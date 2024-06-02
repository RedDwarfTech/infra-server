use validator::Validate;

#[derive(serde::Deserialize, Validate)]
pub struct ChangePwdReq {
    #[validate(length(min=1))]
    #[serde(rename = "oldPassword")]
    pub old_password: String,
    #[validate(length(min=1))]
    #[serde(rename = "newPassword")]
    pub new_password: String,
}