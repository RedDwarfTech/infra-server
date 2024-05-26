use validator::Validate;

#[derive(serde::Deserialize, Validate)]
pub struct UserQueryParams {
    #[validate(range(min=1))]
    pub id: i64,
}