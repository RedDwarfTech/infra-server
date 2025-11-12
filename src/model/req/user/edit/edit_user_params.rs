use utoipa::ToSchema;
use validator::Validate;

#[derive(serde::Deserialize, Validate, ToSchema)]
pub struct EditUserParams {
    #[validate(length(min=1))]
    pub nickname: String,
}