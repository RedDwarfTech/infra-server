// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use std::fmt::Display;

use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;
use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::offset::Utc;

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

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = iap_product)]
pub struct IapProduct {
    pub id: i64,
    pub product_id: i32,
    pub product_type: i32,
    pub online_status: i32,
    pub created_time: i64,
    pub updated_time: i64,
    pub product_title: String,
    pub description: String,
    // https://stackoverflow.com/questions/55783064/the-trait-dieselexpression-is-not-implemented-for-bigdecimalbigdecimal
    pub price: BigDecimal,
    pub raw_price: BigDecimal,
    pub currency_code: Option<String>,
    pub app_id: String,
    pub sort: i32,
    pub deleted: i32,
    pub amount: i32,
    pub period: i32,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = app_map)]
pub struct AppMap {
    pub id: i64,
    pub app_id: String,
    pub third_app_id: String,
    pub third_channel: i32,
    pub created_time: i64,
    pub updated_time: i64,
    pub login_redirect_url: String,
    pub login_success_redirect_url: String,
    pub app_private_key: String,
    pub app_public_key: String,
    pub payed_redirect_url: Option<String>,
    pub notify_url: Option<String>,
    pub qr_pay_model: i16,
    pub app_secret: Option<String>,
    pub app_private_key_pkcs1: String,
    pub app_public_key_pkcs1: String,
    pub alipay_public_key: String
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i64,
    pub user_id: i64,
    pub total_price: BigDecimal,
    pub order_status: i32,
    pub third_app_id: String,
    pub app_id: String,
    pub pay_channel: i32,
    pub created_time: i64,
    pub updated_time: i64,
    pub qr_pay_model: i16,
    pub subject: String,
    pub product_code: String,
    pub order_id: String,
    pub seller_id: String,
    pub deleted: i16
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = order_items)]
pub struct OrderItem {
    pub order_id: String,
    pub iap_product_id: i64,
    pub quantity: i32,
    pub price: BigDecimal,
    pub created_time: i64,
    pub updated_time: i64,
    pub id: i64,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = user_sub)]
pub struct UserSub {
    pub id: i64,
    pub app_id: String,
    pub product_id: i32,
    pub iap_product_id: i64,
    pub created_time: i64,
    pub updated_time: i64,
    pub user_id: i64,
    pub sub_start_time: i64,
    pub sub_end_time: i64,
    pub enabled: i16,
    pub order_id: String,
    pub sub_start: DateTime<Utc>,
    pub sub_end: DateTime<Utc>,
}

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = payments)]
pub struct Payment {
    pub id: i64,
    pub payment_id: String,
    pub order_id: String,
    pub amount: BigDecimal,
    pub status: i32,
    pub created_time: i64,
    pub updated_time: i64,
}