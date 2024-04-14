use validator::Validate;

#[derive(serde::Deserialize, Validate)]
pub struct LoginReq {
    pub name: String,
    #[validate(length(min = 1))]
    pub phone: String,
    #[validate(length(min = 1))]
    pub password: String,
    #[validate(length(min = 1))]
    pub app_id: String,
    #[validate(length(min = 1))]
    pub device_id: String,
}