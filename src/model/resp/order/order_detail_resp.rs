use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use crate::model::diesel::dolphin::custom_dolphin_models::Order;

#[derive(Deserialize, Serialize, Default, Clone)]
#[allow(non_snake_case)]
pub struct OrderDetailResp {
    pub orderId: String,
    pub orderStatus: i32,
    pub totalPrice: BigDecimal,
    pub subject: String,
    pub payChannel: i32,
    pub createdTime: i64,
}

impl From<&Order> for OrderDetailResp {
    fn from(order: &Order) -> Self {
        Self {
            orderId: order.order_id.clone(),
            orderStatus: order.order_status,
            totalPrice: order.total_price.clone(),
            subject: order.subject.clone(),
            payChannel: order.pay_channel,
            createdTime: order.created_time,
        }
    }
}
