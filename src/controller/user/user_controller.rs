use std::time::{SystemTime, UNIX_EPOCH};

use crate::common::cache::user_cache::store_login_user;
use crate::composite::user::user_comp::get_cached_user;
use crate::model::diesel::custom::oauth::oauth_add::OauthAdd;
use crate::model::diesel::dolphin::custom_dolphin_models::User;
use crate::service::app::app_service::{query_app_by_app_id, query_cached_app};
use crate::service::oauth::oauth_service::insert_refresh_token;
use crate::service::user::user_service::query_user_by_product_id;
use actix_web::{get, post, put, web, Responder};
use chrono::Local;
use log::error;
use rust_wheel::common::util::security_util::get_sha;
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;
use rust_wheel::common::wrapper::actix_http_resp::box_error_actix_rest_response;
use rust_wheel::config::app::app_conf_reader::get_app_config;
use rust_wheel::config::cache::redis_util::{incre_redis_key, set_str, sync_get_str};
use rust_wheel::model::response::user::login_response::LoginResponse;
use rust_wheel::model::user::jwt_auth::create_access_token;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use rust_wheel::model::user::web_jwt_payload::WebJwtPayload;
use sha256::digest;
use uuid::Uuid;
use crate::model::req::user::login::login_req::LoginReq;

/// User login
///
/// user login
#[utoipa::path(
    context_path = "/infra/user/login",
    path = "/",
    responses(
        (status = 200, description = "support user login")
    )
)]
#[post("/login")]
pub async fn login(form: actix_web_validator::Json<LoginReq>) -> impl Responder {
    // https://stackoverflow.com/questions/72748775/error-the-trait-handler-is-not-implemented-for-fn-httpresponse
    let login_failed_key = get_app_config("infra.login_failed_key");
    let user_failed_key = format!("{}:{}", login_failed_key, form.0.phone);
    let app_str = sync_get_str(&user_failed_key);
    if app_str.is_some() && app_str.unwrap().parse::<i32>().unwrap() > 3 {
        return box_error_actix_rest_response(
            "LOGIN_FAILED_TOO_MUCH",
            "0030010002".to_owned(),
            "登录错误次数过多".to_owned(),
        );
    }
    let app_info = query_app_by_app_id(&form.0.app_id);
    let single_user: User = query_user_by_product_id(&form.0, &app_info.product_id);
    let pwd_salt = single_user.salt.clone();
    let sha_password = get_sha(String::from(&form.password), &pwd_salt);
    if sha_password.eq(&single_user.pwd.as_str()) {
        let now = SystemTime::now();
        // 过期时间为当前时间加上 1 小时
        let exp = now
            .checked_add(std::time::Duration::new(7200, 0))
            .expect("Unable to calculate expiration time")
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!");
        let exp_timestamp = exp.as_secs() as usize;
        let rd_user = WebJwtPayload {
            userId: single_user.id.clone(),
            deviceId: form.0.device_id.clone(),
            appId: form.0.app_id.clone(),
            lt: 1,
            et: 0,
            pid: app_info.product_id,
            exp: exp_timestamp,
        };
        let uuid = Uuid::new_v4();
        let access_token = create_access_token(&rd_user);
        let login_resp: LoginResponse = LoginResponse {
            registerTime: single_user.register_time.clone(),
            refreshToken: uuid.to_string(),
            accessToken: access_token,
            nickname: single_user.nickname.to_string(),
        };
        let now = Local::now();
        let future_time = now + chrono::Duration::days(7);
        let future_timestamp = future_time.timestamp();
        let oauth = OauthAdd {
            refresh_token: digest(uuid.to_string()),
            user_id: single_user.id.clone(),
            expire_date: future_timestamp,
            device_id: form.0.device_id.clone(),
            app_id: form.0.app_id,
        };
        store_login_user(&rd_user, &single_user, &app_info);
        insert_refresh_token(&oauth);
        return box_actix_rest_response(login_resp);
    } else {
        increase_failed_count(form.0.phone);
        //if let Err(e) = incre_resp {
        //    error!("increase login failed failed: {}", e);
        //}
        return box_error_actix_rest_response(
            "LOGIN_INFO_NOT_MATCH",
            "0030010001".to_owned(),
            "登录信息不匹配".to_owned(),
        );
    }
}

fn increase_failed_count(user_name: String) {
    let login_failed_key = get_app_config("infra.login_failed_key");
    let user_failed_key = format!("{}:{}", login_failed_key, user_name);
    let app_str = sync_get_str(&user_failed_key);
    if app_str.is_none() {
        set_str(&user_failed_key, "1", 120);
    } else {
        let incre_result = incre_redis_key(&user_failed_key, 1);
        if let Err(err) = incre_result {
            error!("increment login count failed, {}", err)
        }
    }
}

/// Current user
///
/// current user
#[utoipa::path(
    context_path = "/infra/user/current-user",
    path = "/",
    responses(
        (status = 200, description = "get current user")
    )
)]
#[get("/current-user")]
pub async fn current_user(login_user_info: LoginUserInfo) -> impl Responder {
    let app = query_cached_app(&login_user_info.appId);
    let cur_user = get_cached_user(&login_user_info, &app);
    return box_actix_rest_response(cur_user);
}

/// Change password
///
/// Change password
#[utoipa::path(
    context_path = "/infra/user/change-pwd",
    path = "/",
    responses(
        (status = 200, description = "change password")
    )
)]
#[put("/pwd")]
pub async fn change_passowrd(login_user_info: LoginUserInfo) -> impl Responder {
    let app = query_cached_app(&login_user_info.appId);
    let cur_user = get_cached_user(&login_user_info, &app);
    return box_actix_rest_response(cur_user);
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/user")
        .service(login)
        .service(change_passowrd)
        .service(current_user);
    conf.service(scope);
}
