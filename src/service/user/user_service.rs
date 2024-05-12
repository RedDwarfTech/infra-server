use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::dolphin::custom_dolphin_models::User;

pub fn query_user_by_product_id(user_phone: &String, prod_id: &i32) -> Option<User> {
    use crate::model::diesel::dolphin::dolphin_schema::users::dsl::*;
    let predicate = crate::model::diesel::dolphin::dolphin_schema::users::phone
        .eq(user_phone.clone())
        .and(crate::model::diesel::dolphin::dolphin_schema::users::product_id.eq(prod_id.clone()));
    let db_user = users
        .filter(&predicate)
        .limit(1)
        .first::<User>(&mut get_conn())
        .ok();
    return db_user;
}

pub fn query_user_by_id(u_id: &i64) -> User {
    use crate::model::diesel::dolphin::dolphin_schema::users::dsl::*;
    let predicate = crate::model::diesel::dolphin::dolphin_schema::users::id.eq(u_id);
    let db_user = users
        .filter(&predicate)
        .limit(1)
        .first::<User>(&mut get_conn())
        .expect("query user by id failed");
    return db_user;
}

pub fn add_user(add_u: &UserAdd){
    use crate::model::diesel::dolphin::dolphin_schema::users as users_table;
    let result = diesel::insert_into(users_table::dsl::user_sub)
        .values(add_u)
        .get_result::<User>(connection)
        .expect("failed to add user");
    return result;
}
