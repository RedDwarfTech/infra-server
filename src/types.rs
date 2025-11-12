use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PostRequest {
    pub name: String,
}

#[derive(ToSchema, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PostResponse {
    pub status: String,
}
