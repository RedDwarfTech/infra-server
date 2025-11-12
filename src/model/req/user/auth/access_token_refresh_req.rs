use validator::Validate;
use utoipa::ToSchema;

#[derive(serde::Deserialize, Validate, ToSchema)]
pub struct AccessTokenRefreshReq {
    #[validate(length(min = 1))]
    pub refresh_token: String,
    #[validate(length(min = 1))]
    pub grant_type: String,
    pub scope: Option<String>,
}