use validator::Validate;

#[derive(serde::Deserialize, Validate, Debug)]
#[allow(non_snake_case)]
pub struct SmsVerifyReq {
    #[validate(length(min = 1, max = 64))]
    pub phone: String,
    #[validate(length(min = 1, max = 64))]
    pub verifyCode: String,
}