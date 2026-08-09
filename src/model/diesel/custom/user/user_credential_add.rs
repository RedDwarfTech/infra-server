use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;

#[derive(Insertable, Queryable, QueryableByName, Debug, Serialize, Deserialize, Default, Clone)]
#[diesel(table_name = user_credential)]
pub struct UserCredentialAdd {
    pub user_id: i64,
    pub credential_type: String,
    pub identifier: String,
    pub credential: String,
    pub salt: String,
    pub status: i32,
    pub app_id: String,
    pub product_id: i32,
    pub created_time: i64,
    pub updated_time: i64,
}
