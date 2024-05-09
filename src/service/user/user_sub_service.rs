use crate::diesel::prelude::*;
use crate::model::diesel::custom::user::user_sub_add::UserSubAdd;
use crate::{
    common::db::database::get_conn, model::diesel::dolphin::custom_dolphin_models::UserSub,
};
use diesel::ExpressionMethods;
use rust_wheel::common::util::time_util::get_current_millisecond;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn query_user_sub_by_product_id(pid: &i32, uid: &i64) -> Vec<UserSub> {
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
        .load::<UserSub>(&mut get_conn())
        .expect("query user by id failed");
    return db_user;
}

pub fn insert_user_sub(user_sub: &UserSubAdd, connection: &mut PgConnection) -> UserSub {
    use crate::model::diesel::dolphin::dolphin_schema::user_sub as user_sub_table;
    let result = diesel::insert_into(user_sub_table::dsl::user_sub)
        .values(user_sub)
        .get_result::<UserSub>(connection)
        .expect("failed to add user sub");
    return result;
}

pub fn query_user_sub_by_order_id(out_trans_no: &String) -> Vec<UserSub> {
    use crate::model::diesel::dolphin::dolphin_schema::user_sub as user_sub_table;
    let predicate = user_sub_table::order_id.eq(out_trans_no);
    let db_user = user_sub_table::table
        .filter(&predicate)
        .load::<UserSub>(&mut get_conn())
        .expect("query user by order id failed");
    return db_user;
}

pub fn get_user_sub_expire_time(uid: &i64, pid: &i32) -> UserSub {
    use crate::model::diesel::dolphin::dolphin_schema::user_sub as user_sub_table;
    let predicate = user_sub_table::user_id
        .eq(uid)
        .and(user_sub_table::enabled.eq(1))
        .and(user_sub_table::sub_end_time.gt(get_current_millisecond()))
        .and(user_sub_table::product_id.eq(pid));
    let db_user = user_sub_table::table
        .filter(&predicate)
        .order_by(&user_sub_table::sub_end_time.desc())
        .first::<UserSub>(&mut get_conn())
        .expect("query user by product id failed");
    return db_user;
}
