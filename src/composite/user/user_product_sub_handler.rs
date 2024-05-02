use rust_wheel::model::enums::{
    order::rd_order_status::RdOrderStatus, pay::apple_pay_product_type::ApplePayProductType,
};

use crate::{
    model::diesel::dolphin::custom_dolphin_models::{IapProduct, UserSub},
    service::{
        app::app_service::query_cached_app, goods::goods_service::query_goods_by_id, order::{
            order_item_service::query_order_item_by_oid,
            order_service::{query_order_by_out_trans_no, update_order_status},
        }
    },
};
use log::{error, warn};

pub fn product_pay_success(out_trans_no: &String) {
    let db_order = query_order_by_out_trans_no(&out_trans_no);
    let o_items = query_order_item_by_oid(&db_order.order_id);
    if o_items.len() != 1 {
        error!("order item size error,{}", out_trans_no);
        return;
    }
    if db_order.order_status == RdOrderStatus::WaitingForPayment as i32 {
        update_order_status(&db_order.id, RdOrderStatus::PAID as i32);
    }
    let iap_id = o_items.get(0).unwrap().iap_product_id;
    let goods = query_goods_by_id(&iap_id);
    handle_sub_by_type(&goods, db_order.user_id, out_trans_no);
}

pub fn handle_sub_by_type(iap: &IapProduct, uid: i64, out_trans_no: &String) {
    if iap.product_type == ApplePayProductType::NonSubscription as i32 {
        handle_non_subscribe(iap, uid, out_trans_no);
    }
}

pub fn handle_non_subscribe(iap: &IapProduct, uid: i64, out_trans_no: &String) {
    warn!("start handle non subscribe");
    let app = query_cached_app(&iap.app_id);

    // let user_sub = UserSub{};


}
