use validator::Validate;

#[derive(serde::Deserialize, Validate)]
#[allow(non_snake_case)]
pub struct AlipayOrderReq {
    pub productId: i64,
}