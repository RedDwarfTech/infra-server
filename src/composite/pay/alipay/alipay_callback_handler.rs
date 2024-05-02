use diesel::Connection;
use log::{error, warn};
use rust_wheel::model::enums::rd_pay_status::RdPayStatus;
use serde_json::from_str;
use std::collections::HashMap;

use crate::{
    common::db::database::get_conn, model::diesel::custom::pay::payment_add::PaymentAdd,
    service::pay::sys::payment_service::save_payment,
};

pub fn handle_pay_callback(query_string: &str) {
    let params: HashMap<String, String> = match from_str(query_string) {
        Ok(parsed_params) => parsed_params,
        Err(_) => HashMap::new(),
    };
    warn!(
        "query_str: {}, params: {}",
        query_string,
        serde_json::to_string(&params).unwrap_or_default()
    );
    let cb_order_id = params.get("out_trade_no").unwrap();
    let cb_payment_id = params.get("trade_no").unwrap();
    let total_amount = params.get("total_amount").unwrap();
    let payment_new = PaymentAdd {
        payment_id: cb_payment_id.to_string(),
        order_id: cb_order_id.to_string(),
        amount: total_amount.parse().expect("Failed to parse BigDecimal"),
        status: RdPayStatus::Success as i32,
    };
    let mut connection = get_conn();
    let result: Result<Option<&str>, diesel::result::Error> = connection.transaction(|conn| {
        save_payment(&payment_new, conn);

        Ok(None)
    });
    if let Err(e) = result {
        error!("handle pay callback failed, {}", e);
    }
}
