use validator::Validate;

#[derive(serde::Deserialize, Validate)]
#[allow(non_snake_case)]
pub struct SmsReq {
    pub phone: String,
    pub app_id: String,
    pub tpl_code: String,
}