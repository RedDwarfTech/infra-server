use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use crate::model::diesel::dolphin::custom_dolphin_models::Order;

#[derive(Deserialize, Serialize, Default, Clone)]
#[allow(non_snake_case)]
pub struct OrderPageResp {
    pub orderId: String,
    pub orderStatus: i32,
    pub totalPrice: BigDecimal,
    pub createdTime: i64
}

impl From<&Order> for OrderPageResp {
    fn from(order: &Order) -> Self {
        Self {
            orderId: order.order_id.clone(),
            orderStatus: order.order_status,
            totalPrice: order.total_price.clone(),
            createdTime: order.created_time,   
        }
    }
}
