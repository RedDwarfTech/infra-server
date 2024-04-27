use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    model::{
        req::user::auth::access_token_refresh_req::AccessTokenRefreshReq,
        resp::auth::auth_resp::AuthResp,
    },
    service::{
        app::app_service::query_cached_app,
        oauth::oauth_service::{query_refresh_token, update_refresh_token_exp_time},
    },
};
use actix_web::{get, web, Responder};
use rust_wheel::{
    common::wrapper::actix_http_resp::box_actix_rest_response,
    model::user::{jwt_auth::create_access_token, web_jwt_payload::WebJwtPayload},
};
use sha256::digest;

/// Create order
///
/// create order
#[utoipa::path(
    context_path = "/infra/alipay/pay",
    path = "/",
    responses(
        (status = 200, description = "create order")
    )
)]
#[get("/createOrder")]
pub async fn create_order(
    form: actix_web_validator::Json<AccessTokenRefreshReq>,
) -> impl Responder {
    // why we should use sha256?
    // even the user get the shd256 token, it could not used to refresh the token
    let input = String::from(&form.0.refresh_token);
    let val = digest(input);
    let db_refresh_token = query_refresh_token(&val);
    let app = query_cached_app(&db_refresh_token.app_id);
    let now = SystemTime::now();
    // 过期时间为当前时间加上 1 小时
    let exp = now
        .checked_add(std::time::Duration::new(7200, 0))
        .expect("Unable to calculate expiration time")
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH!");
    let exp_timestamp = exp.as_secs() as usize;
    let rd_user = WebJwtPayload {
        userId: db_refresh_token.user_id.clone(),
        deviceId: db_refresh_token.device_id.clone(),
        appId: app.app_id,
        lt: 1,
        et: 0,
        pid: app.product_id,
        exp: exp_timestamp,
    };
    let access_token = create_access_token(&rd_user);
    update_refresh_token_exp_time(&db_refresh_token);
    let resp = AuthResp::from(access_token);
    return box_actix_rest_response(resp);
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/alipay/pay").service(create_order);
    conf.service(scope);
}
