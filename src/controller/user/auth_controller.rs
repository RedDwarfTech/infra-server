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
    HASHMAP,
};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use log::error;
use rust_wheel::{
    common::{
        error::jwt_token_error::JwtTokenError, wrapper::actix_http_resp::box_actix_rest_response,
    },
    model::user::{
        jwt_auth::{
            create_access_token, get_auth_token_from_traefik, get_forward_url_path,
            verify_jwt_token,
        },
        web_jwt_payload::WebJwtPayload,
    },
};
use sha256::digest;

/// Refresh access token
///
/// Refresh access token
#[utoipa::path(
    context_path = "/infra/auth/access_token/refresh",
    path = "/",
    responses(
        (status = 200, description = "refresh access token")
    )
)]
// https://developers.google.com/search/docs/crawling-indexing/url-structure
// the stackoverflow also use '-' rather than '_'
#[post("/access-token/refresh")]
pub async fn refresh_access_token(
    form: actix_web_validator::Json<AccessTokenRefreshReq>,
) -> impl Responder {
    // why we should use sha256?
    // even the user get the shd256 token, it could not used to refresh the token
    let input = String::from(&form.0.refresh_token);
    let val = digest(input);
    let db_refresh_token = query_refresh_token(&val);
    let app = query_cached_app(&db_refresh_token.app_id);
    let now = SystemTime::now();
    // the expire time is 1 hour
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

/// Verify access token
///
/// Verify access token
#[utoipa::path(
    context_path = "/infra/auth/access_token",
    path = "/",
    responses(
        (status = 200, description = "verify access token")
    )
)]
#[get("/access_token/verify")]
pub async fn verify_access_token(req: HttpRequest) -> impl Responder {
    let forward_url = get_forward_url_path(&req);
    if forward_url.is_some() {
        if HASHMAP.contains_key(forward_url.unwrap()) {
            return box_actix_rest_response("ok");
        }
    }
    let access_token = get_auth_token_from_traefik(&req);
    if access_token.is_empty() {
        return HttpResponse::Unauthorized().finish();
    }
    let valid = verify_jwt_token(&access_token.as_str());
    match valid {
        JwtTokenError::Valid => {
            return box_actix_rest_response("ok");
        }
        JwtTokenError::Invalid => {
            error!("Invalid access token, token:{}", access_token);
            return HttpResponse::Unauthorized().finish();
        }
        JwtTokenError::Expired => {
            error!("access token expired, token:{}", access_token);
            return HttpResponse::Unauthorized().finish();
        }
        JwtTokenError::OtherError => {
            error!("other issue, token:{}", access_token);
            return HttpResponse::Unauthorized().finish();
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/auth")
        .service(verify_access_token)
        .service(refresh_access_token);
    conf.service(scope);
}
