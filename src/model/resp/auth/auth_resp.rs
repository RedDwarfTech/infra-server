use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
#[allow(non_snake_case)]
pub struct AuthResp {
    pub accessToken: String,
}

impl From<String> for AuthResp {
    fn from(token: String) -> Self {
        Self { accessToken: token }
    }
}
