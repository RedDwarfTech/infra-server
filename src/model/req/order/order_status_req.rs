use validator::Validate;

#[derive(serde::Deserialize, Validate)]
#[allow(non_snake_case)]
pub struct OrderStatusReq {
    pub orderId: String,
}