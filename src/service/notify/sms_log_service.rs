use diesel::{BoolExpressionMethods, ExpressionMethods};
use log::error;
use rust_wheel::common::util::time_util::{end_of_today, start_of_today};

use crate::common::db::database::get_conn;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::custom::notify::sms_log_add::SmsLogAdd;
use crate::model::diesel::dolphin::custom_dolphin_models::SmsLog;
use crate::diesel::QueryDsl;

pub fn save_sms_log(sms_log: &SmsLogAdd) {
    use crate::model::diesel::dolphin::dolphin_schema::sms_log as query_table;
    let result = diesel::insert_into(query_table::dsl::sms_log)
        .values(sms_log)
        .get_result::<SmsLog>(&mut get_conn());
    if let Err(e) = result {
        error!("insert sms log failed: {:?}, sms log: {:?}", e, sms_log);
    }
}

pub fn count_today_sms_log() -> i64 {
    use crate::model::diesel::dolphin::dolphin_schema::sms_log as query_table;
    let end = end_of_today();
    let start = start_of_today();
    let predicate = query_table::created_time
        .lt(end)
        .and(query_table::created_time.gt(start));
    let query_resp = query_table::table.filter(&predicate)
        .count()
        .get_result(&mut get_conn())
        .unwrap();
    return query_resp;
}
