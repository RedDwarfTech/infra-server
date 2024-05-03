use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::custom_dolphin_models::AppMap;

pub fn query_app_map_by_app_id(filter_app_id: &String, pay_type: i32) -> AppMap {
    use crate::model::diesel::dolphin::dolphin_schema::app_map as app_map_table;
    let predicate = app_map_table::app_id
        .eq(filter_app_id)
        .and(app_map_table::third_channel.eq(pay_type));
    let err_msg = format!("{}{}", "query app map failed, id: ", filter_app_id);
    let app_map_result = app_map_table::table
        .filter(&predicate)
        .first::<AppMap>(&mut get_conn())
        .expect(&err_msg);
    return app_map_result;
}

pub fn query_app_map_by_third_app_id(t_app_id: &String, pay_type: i32) -> AppMap {
    use crate::model::diesel::dolphin::dolphin_schema::app_map as app_map_table;
    let predicate = app_map_table::third_app_id
        .eq(t_app_id)
        .and(app_map_table::third_channel.eq(pay_type));
    let err_msg = format!("{}{}", "query app map failed, id: ", t_app_id);
    let app_map_result = app_map_table::table
        .filter(&predicate)
        .first::<AppMap>(&mut get_conn())
        .expect(&err_msg);
    return app_map_result;
}