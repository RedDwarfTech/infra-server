use serde::{Deserialize, Serialize};
use crate::model::diesel::dolphin::dolphin_schema::*;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = oauth2_refresh_token)]
pub struct OauthAdd {
    pub refresh_token: String,
    pub user_id: i64,
    pub expire_date: i64,
    pub device_id: String,
    pub app_id: String,
}