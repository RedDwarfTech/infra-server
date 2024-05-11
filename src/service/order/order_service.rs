use rust_wheel::common::query::pagination::Paginate;
use rust_wheel::common::util::model_convert::map_pagination_res;
use rust_wheel::model::enums::rd_deleted_status::RdDeletedStatus;
use rust_wheel::model::response::pagination_response::PaginationResponse;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::custom::order::order_add::OrderAdd;
use crate::model::diesel::custom::order::order_item_add::OrderItemAdd;
use crate::model::diesel::dolphin::custom_dolphin_models::Order;
use crate::model::req::order::user_order_query_params::UserOrderQueryParams;
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

pub fn update_order_status(oid: &i64, o_status: i32, connection: &mut PgConnection) {
    use crate::model::diesel::dolphin::dolphin_schema::orders as order_table;
    let predicate = order_table::id.eq(oid);
    diesel::update(order_table::table.filter(predicate))
        .set(order_table::order_status.eq(o_status))
        .get_result::<Order>(connection)
        .expect("udpate order status failed");
}

pub fn get_user_order_page(
    params: &UserOrderQueryParams,
    login_user_info: &LoginUserInfo,
) -> PaginationResponse<Vec<Order>> {
    use crate::model::diesel::dolphin::dolphin_schema::orders as orders_table;
    let mut query = orders_table::table.into_boxed::<diesel::pg::Pg>();
    query = query.filter(orders_table::deleted.eq(RdDeletedStatus::Normal as i16));
    query = query.filter(orders_table::user_id.eq(login_user_info.userId));
    let query = query
        .paginate(params.page_num.unwrap_or(1).clone())
        .per_page(params.page_size.unwrap_or(9).clone());
    let page_result: QueryResult<(Vec<Order>, i64, i64)> =
        query.load_and_count_pages_total::<Order>(&mut get_conn());
    let page_map_result = map_pagination_res(
        page_result,
        params.page_num.unwrap_or(1),
        params.page_size.unwrap_or(10),
    );
    return page_map_result;
}
