use crate::diesel::prelude::*;
use crate::model::diesel::custom::order::order_add::OrderAdd;
use crate::model::diesel::custom::order::order_item_add::OrderItemAdd;
use crate::model::diesel::dolphin::custom_dolphin_models::Order;
use crate::service::order::order_item_service::create_order_item;

pub fn create_new_order(new_order: &OrderAdd, connection: &mut PgConnection, order_item: &OrderItemAdd) -> Order {
    use crate::model::diesel::dolphin::dolphin_schema::orders as orders_table;
    let result = diesel::insert_into(orders_table::dsl::orders)
        .values(new_order)
        .get_result::<Order>(connection)
        .expect("failed to add new order");
    create_order_item(&order_item, connection);
    return result;
}
