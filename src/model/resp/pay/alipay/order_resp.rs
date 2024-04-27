use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Clone)]
#[allow(non_snake_case)]
pub struct OrderResp {
    pub formText: String,
    pub orderId: String,
    pub price: String,
}