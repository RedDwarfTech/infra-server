use utoipa::ToSchema;
use validator::Validate;

#[derive(serde::Deserialize, Validate, Debug, ToSchema)]
#[allow(non_snake_case)]
pub struct EmailRegReq {
    #[validate(length(min = 1, max = 128))]
    pub email: String,
    #[validate(length(max = 32))]
    pub phone: Option<String>,
    #[validate(length(min = 1))]
    pub password: String,
    #[validate(length(min = 1))]
    #[serde(rename = "appId")]
    pub app_id: String,
    #[validate(length(min = 1))]
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[validate(length(min = 1))]
    #[serde(rename = "verifyCode")]
    pub verify_code: String,
}
