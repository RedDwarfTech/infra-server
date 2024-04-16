// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
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
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = apps)]
pub struct App {
    pub id: i32,
    pub app_name: String,
    pub remark: String,
    pub created_time: i64,
    pub updated_time: i64,
    pub user_count: i32,
    pub online_status: i32,
    pub online_time: Option<i64>,
    pub app_abbr: String,
    pub app_id: String,
    pub app_tag: Option<String>,
    pub auth_mode: i16,
    pub product_id: i32,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = oauth2_refresh_token)]
pub struct Oauth2RefreshToken {
    pub id: i64,
    pub refresh_token: String,
    pub user_id: i64,
    pub expire_date: i64,
    pub created_time: i64,
    pub updated_time: i64,
    pub replaced_by: Option<String>,
    pub revoked_by_ip: Option<String>,
    pub revoked_date: Option<String>,
    pub device_id: String,
    pub app_type: Option<i32>,
    pub auth_mode: Option<i32>,
    pub app_id: String,
}