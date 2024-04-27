use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;
use bigdecimal::BigDecimal;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = order_items)]
pub struct OrderItemAdd {
    pub order_id: String,
    pub iap_product_id: i64,
    pub price: BigDecimal,
}