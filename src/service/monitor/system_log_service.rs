use log::error;
use rust_wheel::common::util::time_util::get_current_millisecond;

use crate::common::db::database::get_conn;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::custom::system_log::system_log_add::SystemLogAdd;
use crate::model::diesel::dolphin::custom_dolphin_models::SystemLog;

pub fn save_system_log(system_log: &SystemLogAdd) {
    use crate::model::diesel::dolphin::dolphin_schema::system_log as query_table;
    let result = diesel::insert_into(query_table::dsl::system_log)
        .values(system_log)
        .get_result::<SystemLog>(&mut get_conn());
    if let Err(e) = result {
        error!("insert system log failed: {:?}, system log: {:?}", e, system_log);
    }
}

pub fn save_system_log_with_now(system_log: &mut SystemLogAdd) {
    // 若未传日志时间，使用服务端当前时间
    if system_log.log_time == 0 {
        system_log.log_time = get_current_millisecond();
    }
    save_system_log(system_log);
}