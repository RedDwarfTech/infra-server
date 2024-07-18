use serde::Serialize;
use serde::Deserialize;
use crate::model::diesel::dolphin::dolphin_schema::*;

#[derive(Insertable,Queryable,QueryableByName,Debug,Serialize,Deserialize,Default,Clone)]
#[diesel(table_name = sms_log)]
pub struct SmsLogAdd {
    pub service: String,
    pub text: Option<String>,
    pub template_code: String,
    pub phone: Option<String>,
    pub request_id: Option<String>,
    pub biz_id: Option<String>,
}