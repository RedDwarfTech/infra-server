use serde::{Deserialize, Serialize};
use crate::model::diesel::dolphin::custom_dolphin_models::Order;

#[derive(Deserialize, Serialize, Default, Clone)]
#[allow(non_snake_case)]
pub struct OrderStatusResp {
    pub orderId: String,
    pub orderStatus: i32,
}

impl From<&Order> for OrderStatusResp {
    fn from(order: &Order) -> Self {
        Self {
            orderId: order.order_id.clone(),
            orderStatus: order.order_status   
        }
    }
}
