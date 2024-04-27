use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;
use bigdecimal::BigDecimal;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = orders)]
pub struct OrderAdd {
    pub user_id: i64,
    pub total_price: BigDecimal,
    pub third_app_id: String,
    pub app_id: String,
    pub pay_channel: i32,
    pub qr_pay_model: i16,
    pub subject: String,
    pub product_code: String,
    pub order_id: String,
    pub seller_id: String,
}