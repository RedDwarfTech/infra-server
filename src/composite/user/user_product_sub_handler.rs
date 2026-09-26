use chrono::{TimeZone, Utc};
use diesel::PgConnection;
use rust_wheel::{
    common::util::time_util::get_current_millisecond,
    config::cache::redis_util::del_redis_key,
    model::enums::{
        order::rd_order_status::RdOrderStatus,
        pay::{apple_pay_product_type::ApplePayProductType, pay_peroid_type::PayPeroidType},
    },
};

use crate::{
    common::cache::user_cache::get_rd_user_cached_key,
    model::diesel::{
        custom::user::user_sub_add::UserSubAdd, dolphin::custom_dolphin_models::IapProduct,
    },
    service::{
        app::app_service::query_cached_app,
        goods::goods_service::query_goods_by_id,
        order::{
            order_item_service::query_order_item_by_oid,
            order_service::{query_order_by_out_trans_no, update_order_status},
        },
        user::user_sub_service::{
            insert_user_sub, query_newest_user_sub_by_product_id, query_user_sub_by_order_id,
        },
    },
};
use log::{error, warn};

pub fn product_pay_success(out_trans_no: &String, connection: &mut PgConnection) {
    let db_order = query_order_by_out_trans_no(&out_trans_no);
    let o_items = query_order_item_by_oid(&db_order.order_id);
    if o_items.len() != 1 {
        error!("order item size error,{}", out_trans_no);
        return;
    }
    if db_order.order_status == RdOrderStatus::WaitingForPayment as i32 {
        update_order_status(&db_order.id, RdOrderStatus::PAID as i32, connection);
    }
    let iap_id = o_items.get(0).unwrap().iap_product_id;
    let goods = query_goods_by_id(&iap_id);
    handle_sub_by_type(&goods, db_order.user_id, out_trans_no, connection);
}

pub fn handle_sub_by_type(
    iap: &IapProduct,
    uid: i64,
    out_trans_no: &String,
    connection: &mut PgConnection,
) {
    // Auto-renewing (3) and non-renewing (4) products are both time limited, so
    // a paid one entitles the user to a period straight away. The two only
    // differ once the platform starts sending renewal notifications, which is
    // not implemented yet. Restricting this to (4) meant a paid (3) product
    // flipped the order to PAID while granting no subscription at all.
    // Consumable (1) and non-consumable (2) products are one-off purchases with
    // no period, so they do not belong in `user_sub`.
    if iap.product_type == ApplePayProductType::SUBSCRIPTION as i32
        || iap.product_type == ApplePayProductType::NonSubscription as i32
    {
        handle_non_subscribe(iap, uid, out_trans_no, connection);
        return;
    }
    warn!(
        "no subscription granted, unsupported product_type:{}, iap_product_id:{}, out_trans_no:{}",
        iap.product_type, iap.id, out_trans_no
    );
}

pub fn handle_non_subscribe(
    iap: &IapProduct,
    uid: i64,
    out_trans_no: &String,
    connection: &mut PgConnection,
) {
    let mut u_sub = UserSubAdd::default();
    u_sub.app_id = iap.app_id.clone();
    u_sub.iap_product_id = iap.id;
    u_sub.user_id = uid;
    u_sub.product_id = iap.product_id;
    u_sub.order_id = out_trans_no.clone();
    let user_subs = query_newest_user_sub_by_product_id(&iap.product_id, &uid);
    if user_subs.is_none() {
        warn!("new sub, pid:{},uid:{}", iap.product_id, &uid);
        // new subscribe
        let start = get_current_millisecond();
        //u_sub.sub_start = Local:;
        u_sub.sub_start_time = start;
        u_sub.sub_start = Utc.timestamp_opt(start / 1000, 0).unwrap()
    } else {
        let max_sub_end_time = user_subs.unwrap().sub_end_time + 1;
        u_sub.sub_start_time = max_sub_end_time;
        u_sub.sub_start = Utc.timestamp_opt(max_sub_end_time / 1000, 0).unwrap()
    }
    let sub_end_time = match get_sub_time(iap, &u_sub.sub_start_time) {
        Some(sub_end_time) => sub_end_time,
        None => return,
    };
    u_sub.sub_end = Utc.timestamp_opt(sub_end_time / 1000, 0).unwrap();
    u_sub.sub_end_time = sub_end_time;
    let u_subs = query_user_sub_by_order_id(&out_trans_no);
    if u_subs.len() == 0 {
        insert_user_sub(&u_sub, connection);
        let app = query_cached_app(&iap.app_id);
        let user_cached_key = get_rd_user_cached_key(&app.app_id, &uid);
        let del_resp = del_redis_key(&user_cached_key);
        if let Err(err) = del_resp {
            error!(
                "delete redis key failed: {}, cache key: {}",
                err, user_cached_key
            );
        }
    }
}

/// Length of the period a paid product grants, starting at `bas_time`.
///
/// Returns `None` for an unknown period, which the caller treats as "grant
/// nothing". This used to answer `-1` instead, writing a subscription that had
/// already expired and so silently denying a purchase the user had paid for.
fn get_sub_time(iap_product: &IapProduct, bas_time: &i64) -> Option<i64> {
    let duration = match PayPeroidType::from(iap_product.period) {
        PayPeroidType::DAY => 86400000,
        PayPeroidType::OneMonth => 2592000000,
        PayPeroidType::ThreeMonth => 7776000000,
        PayPeroidType::SixMonth => 15552000000,
        PayPeroidType::OneYear => 31536000000,
        PayPeroidType::Unknow => {
            error!(
                "unknown pay period:{}, iap_product_id:{}",
                iap_product.period, iap_product.id
            );
            return None;
        }
    };
    return Some(bas_time + duration);
}
