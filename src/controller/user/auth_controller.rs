use crate::{
    model::{
        resp::auth::auth_resp::AuthResp,
        user::auth::access_token_refresh_req::AccessTokenRefreshReq,
    },
    service::{app::app_service::query_cached_app, oauth::oauth_service::query_refresh_token},
};
use actix_web::{web, Responder};
use rust_wheel::{
    common::wrapper::actix_http_resp::box_actix_rest_response,
    model::user::{jwt_auth::create_access_token, web_jwt_payload::WebJwtPayload},
};
use sha256::digest;

pub async fn refresh_access_token(
    form: actix_web_validator::Json<AccessTokenRefreshReq>,
) -> impl Responder {
    // why we should use sha256?
    // even the user get the shd256 token, it could not used to refresh the token
    let input = String::from(&form.0.refresh_token);
    let val = digest(input);
    let token = query_refresh_token(&val);
    let app = query_cached_app(&token.app_id);
    let rd_user = WebJwtPayload {
        userId: token.user_id,
        deviceId: token.device_id,
        appId: app.app_id,
        lt: 1,
        et: 0,
        pid: app.product_id,
    };
    let access_token = create_access_token(&rd_user);
    let resp = AuthResp::from(access_token);
    return box_actix_rest_response(resp);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/infra/auth")
            .route("/refresh-access-token", web::get().to(refresh_access_token)),
    );
}
