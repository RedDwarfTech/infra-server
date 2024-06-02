use validator::Validate;

#[derive(serde::Deserialize, Validate)]
pub struct ResetPwdReq {
    #[validate(length(min=1))]
    #[serde(rename = "appId")]
    pub app_id: String,
    #[validate(length(min=1))]
    pub code: String,
    #[validate(length(min=1))]
    pub password: String,
    #[validate(length(min=1))]
    pub phone: String,
}