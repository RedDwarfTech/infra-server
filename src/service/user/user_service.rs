use crate::common::db::database::get_conn;
use crate::diesel::prelude::*;
use crate::model::diesel::custom::user::user_add::UserAdd;
use crate::model::diesel::dolphin::custom_dolphin_models::User;
use crate::model::req::user::edit::change_pwd_req::ChangePwdReq;
use crate::model::req::user::edit::edit_user_params::EditUserParams;
use actix_web::HttpResponse;
use rust_wheel::common::util::security_util::get_sha;
use rust_wheel::common::util::str_util::generate_random_string;
use rust_wheel::common::util::time_util::get_current_millisecond;
use rust_wheel::common::wrapper::actix_http_resp::{
    box_actix_rest_response, box_error_actix_rest_response,
};
use rust_wheel::model::user::login_user_info::LoginUserInfo;

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

pub fn add_user(add_u: &UserAdd) {
    use crate::model::diesel::dolphin::dolphin_schema::users as users_table;
    diesel::insert_into(users_table::dsl::users)
        .values(add_u)
        .get_result::<User>(&mut get_conn())
        .expect("failed to add user");
}

pub async fn handle_update_nickname(edit_req: &EditUserParams, login_user_info: &LoginUserInfo) {
    use crate::model::diesel::dolphin::dolphin_schema::users as users_table;
    use crate::model::diesel::dolphin::dolphin_schema::users::dsl::*;
    let predicate = users_table::id.eq(login_user_info.userId);
    diesel::update(users_table::table.filter(predicate))
        .set((
            nickname.eq(&edit_req.nickname),
            updated_time.eq(get_current_millisecond()),
        ))
        .get_result::<User>(&mut get_conn())
        .expect("unable to update user nickname");
}

pub fn handle_update_pwd(edit_req: &ChangePwdReq, uid: &i64) {
    use crate::model::diesel::dolphin::dolphin_schema::users as users_table;
    use crate::model::diesel::dolphin::dolphin_schema::users::dsl::*;
    let pwd_salt = generate_random_string(16);
    let salted_pwd = get_sha(edit_req.new_password.clone(), &pwd_salt);
    let predicate = users_table::id.eq(uid);
    diesel::update(users_table::table.filter(predicate))
        .set((
            pwd.eq(&salted_pwd),
            salt.eq(pwd_salt),
            updated_time.eq(get_current_millisecond()),
        ))
        .get_result::<User>(&mut get_conn())
        .expect("unable to update user password");
}

pub fn change_user_pwd(req: &ChangePwdReq, user_info: &User) -> HttpResponse {
    let old_pwd_match = verify_pwd(&req.old_password, user_info);
    if old_pwd_match {
        handle_update_pwd(req, &user_info.id);
        return box_actix_rest_response("ok");
    } else {
        return box_error_actix_rest_response(
            "LOGIN_INFO_NOT_MATCH",
            "0030010001".to_owned(),
            "登陆信息不匹配".to_owned(),
        );
    }
}

fn verify_pwd(old_pwd: &String, user_info: &User) -> bool {
    let sha_pwd = get_sha(old_pwd.to_owned(), &user_info.salt);
    if sha_pwd == user_info.pwd {
        return true;
    }
    return false;
}
