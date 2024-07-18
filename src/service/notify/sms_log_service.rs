use log::error;

use crate::common::db::database::get_conn;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::custom::notify::sms_log_add::SmsLogAdd;
use crate::model::diesel::dolphin::custom_dolphin_models::SmsLog;

pub fn save_sms_log(sms_log: &SmsLogAdd) {
    use crate::model::diesel::dolphin::dolphin_schema::sms_log as query_table;
    let result = diesel::insert_into(query_table::dsl::sms_log)
        .values(sms_log)
        .get_result::<SmsLog>(&mut get_conn());
    if let Err(e) = result {
        error!("insert sms log failed: {:?}, sms log: {:?}", e, sms_log);
    }
}
