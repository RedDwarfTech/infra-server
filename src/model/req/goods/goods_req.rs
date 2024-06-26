use validator::Validate;

#[derive(serde::Deserialize, Validate)]
#[allow(non_snake_case)]
pub struct GoodsReq {
    pub productId: i64
}