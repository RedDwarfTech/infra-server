use std::collections::HashMap;

use crate::common::cache::user_cache::store_login_user;
use crate::composite::user::user_comp::is_valid_password;
use crate::composite::user::user_comp::{
    do_user_reg, get_cached_rd_user, get_cached_user, get_cached_user_by_phone, get_jwt_payload,
    get_rd_user_by_id,
};
use crate::model::diesel::custom::notify::sms_log_add::SmsLogAdd;
use crate::model::diesel::custom::oauth::oauth_add::OauthAdd;
use crate::model::diesel::dolphin::custom_dolphin_models::{App, User};
use crate::model::req::notify::sms::login_sms_verify_req::LoginSmsVerifyReq;
use crate::model::req::notify::sms::sms_req::SmsReq;
use crate::model::req::notify::sms::sms_verify_req::SmsVerifyReq;
use crate::model::req::user::edit::change_pwd_req::ChangePwdReq;
use crate::model::req::user::edit::edit_user_params::EditUserParams;
use crate::model::req::user::login::login_req::LoginReq;
use crate::model::req::user::pwd::reset_pwd_req::ResetPwdReq;
use crate::model::req::user::query::user_query_params::UserQueryParams;
use crate::model::req::user::reg::reg_req::RegReq;
use crate::service::app::app_service::{query_app_by_app_id, query_cached_app};
use crate::service::notify::sms_log_service::save_sms_log;
use crate::service::notify::sms_service::send_sms;
use crate::service::notify::sms_template_service::get_app_sms_tempate;
use crate::service::oauth::oauth_service::insert_refresh_token;
use crate::service::user::user_service::{
    change_user_pwd, handle_update_nickname, query_user_by_product_id, reset_user_pwd,
};
use actix_web::{get, patch, post, put, web, Responder};
use chrono::Local;
use log::error;
use rand::Rng;
use rust_wheel::common::util::security_util::get_sha;
use rust_wheel::common::wrapper::actix_http_resp::box_error_actix_rest_response;
use rust_wheel::common::wrapper::actix_http_resp::{
    box_actix_rest_response, box_err_actix_rest_response,
};
use rust_wheel::config::app::app_conf_reader::get_app_config;
use rust_wheel::config::cache::redis_util::{
    get_str_default, incre_redis_key, set_str, sync_get_str,
};
use rust_wheel::model::error::infra_error::InfraError;
use rust_wheel::model::response::user::login_response::LoginResponse;
use rust_wheel::model::user::jwt_auth::create_access_token;
use rust_wheel::model::user::login_user_info::LoginUserInfo;
use sha256::digest;
use uuid::Uuid;

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
    let single_user_opt: Option<User> =
        query_user_by_product_id(&form.0.phone, &app_info.product_id);
    if single_user_opt.is_none() {
        return box_error_actix_rest_response(
            "USER_NAME_OR_PWD_INCORRECT",
            "0030010004".to_owned(),
            "用户名或密码错误".to_owned(),
        );
    }
    let single_user = single_user_opt.unwrap();
    let pwd_salt = single_user.salt.clone();
    let sha_password = get_sha(String::from(&form.password), &pwd_salt);
    if sha_password.eq(&single_user.pwd.as_str()) {
        let payload = get_jwt_payload(
            &single_user.id,
            &form.0.device_id,
            &form.0.app_id,
            &single_user.product_id,
        );
        let uuid = Uuid::new_v4();
        let access_token = create_access_token(&payload);
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
        store_login_user(&payload, &single_user, &app_info);
        insert_refresh_token(&oauth);
        return box_actix_rest_response(login_resp);
    } else {
        increase_failed_count(form.0.phone);
        //if let Err(e) = incre_resp {
        //    error!("increase login failed failed: {}", e);
        //}
        return box_err_actix_rest_response(InfraError::LoginInfoNotMatch);
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
    let cur_user = get_cached_rd_user(&login_user_info, &app);
    return box_actix_rest_response(cur_user);
}

/// Change password
///
/// Change password
#[utoipa::path(
    context_path = "/infra/user/change/pwd",
    path = "/",
    responses(
        (status = 200, description = "change password")
    )
)]
#[patch("/change/pwd")]
pub async fn change_passowrd(
    req: actix_web_validator::Json<ChangePwdReq>,
    login_user_info: LoginUserInfo,
) -> impl Responder {
    if req.old_password == req.new_password {
        return box_err_actix_rest_response(InfraError::NewOldPwdDuplicate);
    }
    let app: App = query_cached_app(&login_user_info.appId);
    let cur_user = get_cached_user(&login_user_info, &app);
    if app.app_id != cur_user.app_id {
        return box_error_actix_rest_response(
            "APP_INFO_NOT_MATCH",
            "0030010007".to_owned(),
            "APPID不匹配".to_owned(),
        );
    }
    return change_user_pwd(&req.0, &cur_user);
}

/// Register user
///
/// Register user
#[utoipa::path(
    context_path = "/infra/user/reg-user",
    path = "/",
    responses(
        (status = 200, description = "Register user")
    )
)]
#[post("/reg")]
pub async fn reg_user(form: actix_web_validator::Json<RegReq>) -> impl Responder {
    let app = query_cached_app(&form.0.app_id);
    return do_user_reg(&form.0, &app);
}

/// Get user
///
/// Get user
#[utoipa::path(
    context_path = "/infra-inner/user/detail",
    path = "/",
    responses(
        (status = 200, description = "get current user")
    )
)]
#[get("/detail")]
pub async fn get_inner_user(params: web::Query<UserQueryParams>) -> impl Responder {
    let cur_user = get_rd_user_by_id(&params.0.id);
    return box_actix_rest_response(cur_user);
}

/// Update nickname
///
/// Update nickname
#[utoipa::path(
    context_path = "/infra-inner/user/nickname",
    path = "/",
    responses(
        (status = 200, description = "change current user nickname")
    )
)]
#[patch("/nickname")]
pub async fn change_nickname(
    params: actix_web_validator::Json<EditUserParams>,
    login_user_info: LoginUserInfo,
) -> impl Responder {
    handle_update_nickname(&params, &login_user_info).await;
    return box_actix_rest_response("ok");
}

/// Send reset verify code
///
/// Send reset verify code
#[utoipa::path(
    context_path = "/infra/user/pwd/send-reset-verify-code",
    path = "/",
    responses(
        (status = 200, description = "change current user nickname")
    )
)]
#[put("/pwd/send-reset-verify-code")]
pub async fn send_reset_pwd_verify_code(
    params: actix_web_validator::Json<LoginSmsVerifyReq>,
) -> impl Responder {
    let caced_key = format!("infra:user:sms:{}", params.0.phone);
    let redis_resp = get_str_default(&caced_key);
    match redis_resp {
        Ok(data) => {
            if data.is_some() {
                return box_actix_rest_response("too freqency, try again later");
            }
        }
        Err(e) => {
            error!("get redis reset info failed,{},params:{:?}", e, params.0);
            return box_actix_rest_response("ok");
        }
    }
    // user not exists
    let cached_app = query_cached_app(&params.0.app_id);
    let user = get_cached_user_by_phone(&params.0.phone, &cached_app);
    if user.is_none() {
        return box_actix_rest_response("ok");
    }
    let sms_tpl = get_app_sms_tempate(&params.0.app_id, &"reset_pwd".to_owned());
    if sms_tpl.is_none() {
        error!("send reset pwd get template is null,{:?}", &params.0);
        return box_actix_rest_response("ok");
    }
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(100000..=999999);
    let sms_req = SmsReq {
        phone: params.0.phone,
        app_id: params.0.app_id,
        tpl_code: sms_tpl.unwrap().sms_code,
    };
    let mut sms_params = HashMap::new();
    sms_params.insert("code", random_number.to_string());
    let send_result = send_sms(&sms_req, sms_params);
    if send_result.is_some() {
        set_str(&caced_key, &random_number.to_string(), 60);
        let result = send_result.unwrap();
        let log = SmsLogAdd {
            service: "reset_pwd".to_string(),
            text: Some("234".to_string()),
            template_code: sms_req.tpl_code,
            phone: Some(sms_req.phone.clone()),
            request_id: Some(result.Code),
            biz_id: Some(result.BizId),
        };
        save_sms_log(&log);
        return box_actix_rest_response("ok");
    }
    return box_actix_rest_response("ok");
}

/// Verify code
///
/// Verify code
#[utoipa::path(
    context_path = "/infra/user/pwd/verify",
    path = "/",
    responses(
        (status = 200, description = "change current user nickname")
    )
)]
#[put("/verify")]
pub async fn send_login_verify_code(
    params: actix_web_validator::Json<SmsVerifyReq>,
) -> impl Responder {
    let caced_key = format!("infra:user:sms:{}", params.0.phone);
    let redis_resp = get_str_default(&caced_key);
    match redis_resp {
        Ok(data) => {
            if data.is_none() {
                return box_err_actix_rest_response(InfraError::DataNotFound);
            }
            if data.unwrap() == params.0.verifyCode {
                return box_actix_rest_response("ok");
            }
            return box_err_actix_rest_response(InfraError::DataNotFound);
        }
        Err(e) => {
            error!(
                "get redis reset to verify failed,{},params:{:?}",
                e, params.0
            );
            return box_err_actix_rest_response(InfraError::DataNotFound);
        }
    }
}

/// Reset password
///
/// Reset password
#[utoipa::path(
    context_path = "/infra/user/pwd/send-verify-code",
    path = "/",
    responses(
        (status = 200, description = "change current user nickname")
    )
)]
#[put("/pwd/reset")]
pub async fn reset_pwd(params: actix_web_validator::Json<ResetPwdReq>) -> impl Responder {
    let caced_key = format!("infra:user:sms:{}", &params.0.phone);
    let redis_resp = get_str_default(&caced_key);
    if let Err(e) = redis_resp.as_ref() {
        error!("get cache failed,{}, params: {:?}", e, params);
        return box_err_actix_rest_response(InfraError::DataNotFound);
    }
    if redis_resp.as_ref().unwrap().is_none() {
        return box_err_actix_rest_response(InfraError::DataNotFound);
    }
    if redis_resp.unwrap().unwrap() != params.0.code {
        return box_err_actix_rest_response(InfraError::SmsVerifyCodeNotMatch);
    }
    let app: App = query_cached_app(&params.0.app_id);
    let cur_user = get_cached_user_by_phone(&params.phone, &app);
    if cur_user.is_none() {
        return box_err_actix_rest_response(InfraError::DataNotFound);
    }
    if !is_valid_password(&params.0.password) {
        return box_err_actix_rest_response(InfraError::PwdNitMatchComplexGuide);
    }
    reset_user_pwd(&params.0, &cur_user.unwrap());
    return box_actix_rest_response("ok");
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/user")
        .service(login)
        .service(change_passowrd)
        .service(reg_user)
        .service(change_nickname)
        .service(send_reset_pwd_verify_code)
        .service(send_login_verify_code)
        .service(reset_pwd)
        .service(current_user);
    conf.service(scope);
    let scope_inner = web::scope("/infra-inner/user").service(get_inner_user);
    conf.service(scope_inner);
}
