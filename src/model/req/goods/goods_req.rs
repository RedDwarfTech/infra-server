use utoipa::ToSchema;
use validator::Validate;

#[derive(serde::Deserialize, Validate, ToSchema)]
#[allow(non_snake_case)]
pub struct GoodsReq {
    pub productId: i64
}