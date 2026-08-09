use utoipa::ToSchema;
use validator::Validate;

#[derive(serde::Deserialize, Validate, Debug, ToSchema)]
#[allow(non_snake_case)]
pub struct EmailVerifyReq {
    #[validate(length(min = 1, max = 128))]
    pub email: String,
    #[validate(length(min = 1, max = 64))]
    pub app_id: String,
}
