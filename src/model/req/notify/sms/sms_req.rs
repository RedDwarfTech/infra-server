use validator::Validate;

#[derive(serde::Deserialize, Validate)]
#[allow(non_snake_case)]
pub struct SmsReq {
    #[validate(length(min = 1, max = 64))]
    pub phone: String,
    #[validate(length(min = 1, max = 64))]
    pub app_id: String,
    #[validate(length(min = 1, max = 64))]
    pub tpl_code: String,
}