use crate::diesel::prelude::*;
use crate::model::diesel::custom::order::order_item_add::OrderItemAdd;
use crate::model::diesel::dolphin::custom_dolphin_models::OrderItem;

pub fn create_order_item(item: &OrderItemAdd, connection: &mut PgConnection){
    use crate::model::diesel::dolphin::dolphin_schema::order_items as order_item_table;
    diesel::insert_into(order_item_table::dsl::order_items)
    .values(item)
    .get_result::<OrderItem>(connection)
    .expect("failed to add new order item");
}