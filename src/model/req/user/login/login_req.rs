use validator::Validate;
use utoipa::ToSchema;

#[derive(serde::Deserialize, Validate, ToSchema)]
pub struct LoginReq {
    #[validate(length(min = 1))]
    pub phone: String,
    #[validate(length(min = 1))]
    pub password: String,
    #[validate(length(min = 1))]
    #[serde(rename = "appId")]
    pub app_id: String,
    #[validate(length(min = 1))]
    #[serde(rename = "deviceId")]
    pub device_id: String,
}