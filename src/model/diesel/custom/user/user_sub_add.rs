use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = user_sub)]
pub struct UserSubAdd {
    pub app_id: String,
    pub product_id: i32,
    pub iap_product_id: i64,
    pub user_id: i64,
    pub sub_start_time: i64,
    pub sub_end_time: i64,
    pub order_id: String,
    pub sub_start: DateTime<Utc>,
    pub sub_end: DateTime<Utc>,
}