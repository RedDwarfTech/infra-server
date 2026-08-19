use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = system_log)]
pub struct SystemLogAdd {
    pub log_time: i64,
    pub content: String,
    pub source: String,
    pub level: String,
    pub app_id: String,
    pub created_time: i64,
}