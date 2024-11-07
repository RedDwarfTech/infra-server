use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = users)]
pub struct UserAdd {
    pub nickname: String,
    pub avatar_url: Option<String>,
    pub phone: String,
    pub updated_time: i64,
    pub created_time: i64,
    pub salt: String,
    pub pwd: String,
    pub sex: Option<i32>,
    pub level_type: Option<String>,
    pub phone_region: Option<String>,
    pub country_code: Option<String>,
    pub user_status: i32,
    pub last_login_time: Option<i64>,
    pub first_login_time: Option<i64>,
    pub app_id: String,
    pub register_time: i64,
    pub apple_iap_product_id: Option<String>,
    pub auto_renew_product_expire_time_ms: Option<i64>,
    pub is_guest: i32,
    pub product_id: i32,
    pub register_ip: Option<String>,
}