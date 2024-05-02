use chrono::Local;
use diesel::PgConnection;
use labrador::current_timestamp;
use rust_wheel::model::enums::{
    order::rd_order_status::RdOrderStatus, pay::apple_pay_product_type::ApplePayProductType,
};

use crate::{
    model::diesel::dolphin::custom_dolphin_models::{IapProduct, User, UserSub},
    service::{
        app::app_service::query_cached_app, goods::goods_service::query_goods_by_id, order::{
            order_item_service::query_order_item_by_oid,
            order_service::{query_order_by_out_trans_no, update_order_status},
        }, user::user_sub_service::{insert_user_sub, query_user_sub_by_order_id, query_user_sub_by_product_id}
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
    handle_sub_by_type(&goods, db_order.user_id, out_trans_no,connection);
}

pub fn handle_sub_by_type(iap: &IapProduct, uid: i64, out_trans_no: &String,connection: &mut PgConnection) {
    if iap.product_type == ApplePayProductType::NonSubscription as i32 {
        handle_non_subscribe(iap, uid, out_trans_no, connection);
    }
}

pub fn handle_non_subscribe(iap: &IapProduct, uid: i64, out_trans_no: &String,connection: &mut PgConnection) {
    warn!("start handle non subscribe");
    let app = query_cached_app(&iap.app_id);
    let mut u_sub = UserSub::default();
    u_sub.app_id = iap.app_id.clone();
    u_sub.iap_product_id = iap.id;
    u_sub.user_id = uid;
    u_sub.order_id = out_trans_no.clone();
    let user_subs = query_user_sub_by_product_id(&iap.product_id, &uid);
    if user_subs.len() == 0 {
        // new subscribe
        let start = current_timestamp();
        //u_sub.sub_start = Local:;
        u_sub.sub_start_time = start;
    } else {
        // let max_sub = 
    }
    // let user_sub = UserSub{};
    let u_subs = query_user_sub_by_order_id(&out_trans_no);
    if u_subs.len() == 0 {
        insert_user_sub(&u_sub, connection);
        warn!("insert user sub")
    }

}
