use crate::{
    common::cache::user_cache::get_user_cached_key,
    model::{
        diesel::{custom::user::user_add::UserAdd, dolphin::custom_dolphin_models::App},
        req::user::reg::reg_req::RegReq,
    },
    service::{
        app::app_service::query_cached_app,
        user::user_service::{add_user, query_user_by_id, query_user_by_product_id},
    },
};
use actix_web::HttpResponse;
use fancy_regex::Regex;
use rust_wheel::{
    common::{
        util::{
            security_util::get_sha, str_util::generate_random_string,
            time_util::get_current_millisecond,
        },
        wrapper::actix_http_resp::{box_actix_rest_response, box_error_actix_rest_response},
    },
    config::cache::redis_util::sync_get_str,
    model::user::{login_user_info::LoginUserInfo, rd_user_info::RdUserInfo},
};

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

pub fn do_user_reg(req: &RegReq, app: &App) -> HttpResponse {
    if !is_valid_password(&req.password) {
        return box_error_actix_rest_response(
            "PWD_NOT_MATCH_COMPLAEX_GUIDE",
            "0030010006".to_owned(),
            "密码不够安全,密码必须包含大写、小写、数字和特殊字符，且长度是8-32位".to_owned(),
        );
    }
    let exists_user = query_user_by_product_id(&req.phone, &app.product_id);
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
    let salted_pwd = get_sha(req.password.clone(), &pwd_salt);
    reg_u.salt = pwd_salt;
    reg_u.pwd = salted_pwd;
    reg_u.nickname = format!("u_{}", generate_random_string(6));
    reg_u.register_time = get_current_millisecond();
    reg_u.first_login_time = Some(get_current_millisecond());
    reg_u.app_id = app.app_id.clone();
    reg_u.product_id = app.product_id;
    reg_u.country_code = req.country_code.clone();
    add_user(&reg_u);
    return box_actix_rest_response("ok");
}

///
/// https://github.com/rust-lang/regex/issues/618
/// https://github.com/rust-lang/regex/discussions/910
/// 
fn is_valid_password(password: &str) -> bool {
    // 正则表达式：密码必须包含大写、小写、数字和特殊字符，且长度是8-32位
    let re = Regex::new(r"^(?=.*[A-Z])(?=.*[a-z])(?=.*\d)(?=.*[!@#$%^&*()-+=]).{8,32}$").unwrap();
    re.is_match(password).expect(&format!("regex match error,{}",password))
}
