use crate::model::{diesel::dolphin::custom_dolphin_models::User, user::login::login_req::LoginReq};
use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;

pub fn query_user_by_product_id(request: &LoginReq, prod_id: &i32) -> User {
    use crate::model::diesel::dolphin::dolphin_schema::users::dsl::*;
    let predicate =
        crate::model::diesel::dolphin::dolphin_schema::users::phone.eq(request.phone.clone())
        .and(crate::model::diesel::dolphin::dolphin_schema::users::product_id.eq(prod_id.clone()));
    let db_user = users
        .filter(&predicate)
        .limit(1)
        .first::<User>(&mut get_conn())
        .expect("query user failed");
    return db_user;
}

pub fn query_user_by_id(u_id: &i64) -> User {
    use crate::model::diesel::dolphin::dolphin_schema::users::dsl::*;
    let predicate =
        crate::model::diesel::dolphin::dolphin_schema::users::id.eq(u_id);
    let db_user = users
        .filter(&predicate)
        .limit(1)
        .first::<User>(&mut get_conn())
        .expect("query user by id failed");
    return db_user;
}
