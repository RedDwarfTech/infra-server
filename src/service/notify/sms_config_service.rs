use diesel::{ExpressionMethods, QueryDsl};
use crate::diesel::prelude::*;
use crate::{common::db::database::get_conn, model::diesel::dolphin::custom_dolphin_models::SmsConfig};

pub fn get_app_sms_config(filter_app_id: &String) -> SmsConfig {
    use crate::model::diesel::dolphin::dolphin_schema::sms_config as query_table;
    let predicate = query_table::app_id
        .eq(filter_app_id);
    let db_order = query_table::table
        .filter(&predicate)
        .limit(1)
        .first::<SmsConfig>(&mut get_conn())
        .expect("query config by app id failed");
    return  db_order;
}
