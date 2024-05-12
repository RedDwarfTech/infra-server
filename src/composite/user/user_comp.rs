use crate::{
    common::cache::user_cache::get_user_cached_key,
    model::{diesel::{custom::user::user_add::UserAdd, dolphin::custom_dolphin_models::{App, User}}, req::user::reg::reg_req::RegReq},
    service::{app::app_service::query_cached_app, user::user_service::{add_user, query_user_by_id, query_user_by_product_id}},
};
use rust_wheel::{
    common::util::time_util::get_current_millisecond, config::cache::redis_util::sync_get_str, model::user::{login_user_info::LoginUserInfo, rd_user_info::RdUserInfo}
};
use regex::bytes::Regex;

pub fn comp_current_user(login_user_info: &LoginUserInfo) -> RdUserInfo {
    let app: App = query_cached_app(&login_user_info.appId);
    let current_u = get_cached_user(login_user_info, &app);
    return current_u;
}

pub fn get_cached_user(login_user_info: &LoginUserInfo, app: &App) -> RdUserInfo {
    let user_cached_key = get_user_cached_key(&app.app_id, &login_user_info.userId);
    let cached_user_info = sync_get_str(&user_cached_key);
    if cached_user_info.is_some() {
        let u_model: RdUserInfo = serde_json::from_str(&cached_user_info.unwrap()).unwrap();
        return u_model;
    }
    let u_info = query_user_by_id(&login_user_info.userId);
    let rd_user = RdUserInfo {
        id: u_info.id,
        nickname: u_info.nickname,
        device_id: login_user_info.deviceId.to_string(),
        app_id: u_info.app_id,
        avatar_url: u_info.avatar_url.unwrap_or_default(),
        auto_renew_product_expire_time_ms: 0,
        app_name: app.app_name.to_string(),
    };
    return rd_user;
}


pub fn do_user_reg(req: &RegReq, app: &App){
    if !is_valid_password(&req.password) {
        return box_error_actix_rest_response(
            "PWD_NOT_MATCH_COMPLAEX_GUIDE",
            "0030010006".to_owned(),
            "密码不够安全".to_owned(),
        );
    } 
    let exists_user = query_user_by_product_id(&req.phone,&app.product_id);
    if exists_user.is_some() {
        return box_error_actix_rest_response(
            "USER_ALREADY_REG",
            "0030010005".to_owned(),
            "用户已注册".to_owned(),
        );
    }
    let mut reg_u = UserAdd::default();
    reg_u.phone = req.phone.clone();
    let pwd_salt = generate_random_string(16);
    let salted_pwd = get_sha(req.password,pwd_salt);
    reg_u.salt = pwd_salt;
    reg_u.password = salted_pwd;
    reg_u.nickname = format!("{}_{}",u, generate_random_string(6));
    reg_u.register_time = get_current_millisecond();
    reg_u.first_login_time = Some(get_current_millisecond());
    reg_u.app_id = app.app_id.clone();
    reg_u.product_id = app.product_id;
    reg_u.country_code = Some(req.country_code);
    add_user(&reg_u);
}

fn is_valid_password(password: &str) -> bool {
    // 正则表达式：密码必须包含大写、小写、数字和特殊字符，且长度是6-32位
    let re = Regex::new(r"^(?=.*[A-Z])(?=.*[a-z])(?=.*\d)(?=.*[!@#$%^&*()-+=]).{6,32}$").unwrap();
    re.is_match(password.as_bytes())
}