use validator::Validate;

#[derive(serde::Deserialize, Validate)]
pub struct EditUserParams {
    #[validate(length(min=1))]
    pub nickname: String,
}