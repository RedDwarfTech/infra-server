use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::custom_dolphin_models::App;

pub fn query_app_by_app_id(filter_app_id: &String) -> App {
    use crate::model::diesel::dolphin::dolphin_schema::apps::dsl::*;
    let predicate =
        crate::model::diesel::dolphin::dolphin_schema::apps::app_id.eq(filter_app_id);
    let db_user = apps
        .filter(&predicate)
        .limit(1)
        .first::<App>(&mut get_conn())
        .expect("query user failed");
    return db_user;
}