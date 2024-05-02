use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::custom::order::order_add::OrderAdd;
use crate::model::diesel::custom::order::order_item_add::OrderItemAdd;
use crate::model::diesel::dolphin::custom_dolphin_models::Order;
use crate::service::order::order_item_service::create_order_item;

pub fn create_new_order(
    new_order: &OrderAdd,
    connection: &mut PgConnection,
    order_item: &OrderItemAdd,
) -> Order {
    use crate::model::diesel::dolphin::dolphin_schema::orders as orders_table;
    let result = diesel::insert_into(orders_table::dsl::orders)
        .values(new_order)
        .get_result::<Order>(connection)
        .expect("failed to add new order");
    create_order_item(&order_item, connection);
    return result;
}

pub fn query_order_by_order_id(o_id: &String, uid: &i64) -> Order {
    use crate::model::diesel::dolphin::dolphin_schema::orders as order_table;
    let predicate = order_table::order_id
        .eq(o_id)
        .and(order_table::user_id.eq(uid));
    let db_order = order_table::table
        .filter(&predicate)
        .limit(1)
        .first::<Order>(&mut get_conn())
        .expect("query order by order id failed");
    return db_order;
}

pub fn query_order_by_out_trans_no(out_trans_no: &String) -> Order {
    use crate::model::diesel::dolphin::dolphin_schema::orders as order_table;
    let predicate = order_table::order_id.eq(out_trans_no);
    let db_order = order_table::table
        .filter(&predicate)
        .limit(1)
        .first::<Order>(&mut get_conn())
        .expect("query order by out trans no failed");
    return db_order;
}

pub fn query_order_by_user_id(uid: &i64) -> Vec<Order> {
    use crate::model::diesel::dolphin::dolphin_schema::orders as order_table;
    let predicate = order_table::user_id.eq(uid);
    let db_order = order_table::table
        .filter(&predicate)
        .limit(20)
        .load::<Order>(&mut get_conn())
        .expect("query order by order id failed");
    return db_order;
}

pub fn update_order_status(oid: &i64, o_status: i32) {
    use crate::model::diesel::dolphin::dolphin_schema::orders as order_table;
    let predicate = order_table::id.eq(oid);
    diesel::update(order_table::table.filter(predicate))
        .set(order_table::order_status.eq(o_status))
        .get_result::<Order>(&mut get_conn())
        .expect("udpate order status failed");
}
