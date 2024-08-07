use crate::{
    composite::user::user_comp::get_jwt_payload,
    model::{
        req::user::auth::access_token_refresh_req::AccessTokenRefreshReq,
        resp::auth::auth_resp::AuthResp,
    },
    service::{
        app::app_service::query_cached_app,
        oauth::oauth_service::{query_refresh_token, update_refresh_token_exp_time},
    },
    VEC,
};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use jsonwebtoken::errors::ErrorKind;
use log::error;
use rust_wheel::{
    common::wrapper::actix_http_resp::{box_actix_rest_response, box_err_actix_rest_response},
    model::{
        error::infra_error::InfraError,
        user::jwt_auth::{
            create_access_token, get_auth_token_from_traefik, get_forward_url_path,
            verify_jwt_token,
        },
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
    if db_refresh_token.is_none() {
        return box_err_actix_rest_response(InfraError::DataNotFound);
    }
    let unwrapped = db_refresh_token.unwrap();
    let app = query_cached_app(&unwrapped.app_id);
    let payload = get_jwt_payload(
        &unwrapped.user_id.clone(),
        &unwrapped.device_id.clone(),
        &app.app_id,
        &app.product_id,
    );
    let access_token = create_access_token(&payload);
    update_refresh_token_exp_time(&unwrapped);
    let resp = AuthResp::from(access_token);
    return box_actix_rest_response(resp);
}

/// Verify access token
///
/// https://stackoverflow.com/questions/8855297/token-expired-json-rest-api-error-code
/// according to the spec rfc6750 - "The OAuth 2.0 Authorization Framework:
/// Bearer Token Usage", https://www.rfc-editor.org/rfc/rfc6750, p.8, section 3.1,
/// resource server should return 401: invalid_token The access token provided is expired, revoked, malformed,
/// or invalid for other reasons. The resource SHOULD respond with the HTTP 401 (Unauthorized) status code.
/// The client MAY request a new access token and retry the protected resource request.
///
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
        if VEC.contains(&forward_url.unwrap().to_string()) {
            return box_actix_rest_response("ok");
        }
    }
    let access_token = get_auth_token_from_traefik(&req);
    if access_token.is_empty() {
        error!(
            "Unauthorized happen,url:{}",
            forward_url.unwrap_or_default()
        );
        return HttpResponse::Unauthorized().finish();
    }
    let valid = verify_jwt_token(&access_token.as_str());
    if valid.is_none() {
        return box_actix_rest_response("ok");
    }
    match valid.unwrap() {
        ErrorKind::InvalidToken => {
            error!("Invalid access token, token:{}", access_token);
            return HttpResponse::Unauthorized().finish();
        }
        ErrorKind::ExpiredSignature => {
            return HttpResponse::Unauthorized().finish();
        }
        _ => {
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
