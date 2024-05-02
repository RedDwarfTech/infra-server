use crate::common::db::database::get_conn;
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

pub fn query_order_item_by_oid(oid: &String) -> Vec<OrderItem> {
    use crate::model::diesel::dolphin::dolphin_schema::order_items as order_item_table;
    let predicate = order_item_table::order_id.eq(oid);
    let db_order_items = order_item_table::table
        .filter(&predicate)
        .load::<OrderItem>(&mut get_conn())
        .expect("query order item by out trans no failed");
    return db_order_items;
}

