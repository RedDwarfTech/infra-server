use crate::common::db::database::get_conn;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::model::diesel::dolphin::custom_dolphin_models::SmsTemplate;
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use log::error;

pub fn get_app_sms_tempate(filter_app_id: &String, filter_code: &String) -> Option<SmsTemplate> {
    use crate::model::diesel::dolphin::dolphin_schema::sms_template as query_table;
    let predicate = query_table::app_id
        .eq(filter_app_id)
        .and(query_table::biz_code.eq(filter_code));
    let query_resp = diesel::QueryDsl::filter(query_table::table, &predicate)
        .limit(1)
        .first::<SmsTemplate>(&mut get_conn());
    match query_resp {
        Ok(data) => {
            return Some(data);
        }
        Err(e) => {
            error!(
                "query sms tempate failed, {},filter_app_id:{}, code:{}",
                e, filter_app_id, filter_code
            );
            return None;
        }
    };
}
