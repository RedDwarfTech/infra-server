use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;
use bigdecimal::BigDecimal;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = payments)]
pub struct PaymentAdd {
    pub payment_id: String,
    pub order_id: String,
    pub amount: BigDecimal,
    pub status: i32
}