use std::time::{SystemTime, UNIX_EPOCH};
use diesel::ExpressionMethods;
use crate::diesel::prelude::*;
use crate::{
    common::db::database::get_conn, model::diesel::dolphin::custom_dolphin_models::UserSub,
};

pub fn query_user_sub_by_product_id(pid: &i32, uid: &i64) -> UserSub {
     let now = SystemTime::now();
     let unix_timestamp: i64 = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64 * 1000 + // 秒数转换为毫秒
        now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .subsec_millis() as i64; // 添加当前的毫秒数
    use crate::model::diesel::dolphin::dolphin_schema::user_sub as user_sub_table;
    let predicate = user_sub_table::product_id
        .eq(pid)
        .and(user_sub_table::sub_end_time.gt(unix_timestamp))
        .and(user_sub_table::user_id.eq(uid))
        .and(user_sub_table::enabled.eq(1));
    let db_user = user_sub_table::table
        .filter(&predicate)
        .first::<UserSub>(&mut get_conn())
        .expect("query user by id failed");
    return db_user;
}
