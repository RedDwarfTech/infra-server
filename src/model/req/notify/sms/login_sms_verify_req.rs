use validator::Validate;

#[derive(serde::Deserialize, Validate, Debug)]
#[allow(non_snake_case)]
pub struct LoginSmsVerifyReq {
    #[validate(length(min = 1, max = 64))]
    pub phone: String,
    #[validate(length(min = 1, max = 64))]
    pub app_id: String,
}