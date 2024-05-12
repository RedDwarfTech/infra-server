use validator::Validate;

#[derive(serde::Deserialize, Validate)]
pub struct RegReq {
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
    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,
}