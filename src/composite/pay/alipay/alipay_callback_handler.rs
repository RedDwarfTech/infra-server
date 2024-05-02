use diesel::Connection;
use log::{error, warn};
use rust_wheel::model::enums::rd_pay_status::RdPayStatus;

use crate::{
    common::db::database::get_conn, composite::user::user_product_sub_handler::product_pay_success, model::{diesel::custom::pay::payment_add::PaymentAdd, pay::callback::alipay_callback::AlipayCallback}, service::pay::sys::payment_service::save_payment
};

pub fn handle_pay_callback(callback: &AlipayCallback) {
    warn!(
        "params: {}",
        serde_json::to_string(&callback).unwrap_or_default()
    );
    let cb_order_id = callback.out_trade_no.clone();
    let cb_payment_id = callback.trade_no.clone();
    let total_amount = callback.total_amount.clone();
    let payment_new = PaymentAdd {
        payment_id: cb_payment_id.to_string(),
        order_id: cb_order_id.to_string(),
        amount: total_amount,
        status: RdPayStatus::Success as i32,
    };
    let mut connection = get_conn();
    let result: Result<Option<&str>, diesel::result::Error> = connection.transaction(|conn| {
        save_payment(&payment_new, conn);
        product_pay_success(&cb_order_id, conn);
        Ok(None)
    });
    if let Err(e) = result {
        error!("handle pay callback failed, {}", e);
    }
}
