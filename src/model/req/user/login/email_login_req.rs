use utoipa::ToSchema;
use validator::Validate;

#[derive(serde::Deserialize, Validate, Debug, ToSchema)]
#[allow(non_snake_case)]
pub struct EmailLoginReq {
    #[validate(length(min = 1, max = 128))]
    pub email: String,
    pub password: Option<String>,
    pub verify_code: Option<String>,
    #[validate(length(min = 1))]
    #[serde(rename = "appId")]
    pub app_id: String,
    #[validate(length(min = 1))]
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[validate(length(min = 1))]
    #[serde(rename = "cfToken")]
    pub cf_token: String,
}
