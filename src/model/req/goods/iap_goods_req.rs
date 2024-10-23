use validator::Validate;

#[derive(serde::Deserialize, Validate)]
#[allow(non_snake_case)]
pub struct IapGoodsReq {
    pub productId: i64,
    lang: Option<i32>
}