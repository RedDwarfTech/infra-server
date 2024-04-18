use crate::{
    model::{
        diesel::dolphin::custom_dolphin_models::App, resp::auth::auth_resp::AuthResp,
        user::auth::access_token_refresh_req::AccessTokenRefreshReq,
    },
    service::{app::app_service::query_cached_app, oauth::oauth_service::query_refresh_token},
};
use actix_web::{web, Responder};
use rust_wheel::{
    common::wrapper::actix_http_resp::box_actix_rest_response,
    model::user::{
        jwt_auth::create_access_token, jwt_payload::JwtPayload, rd_user_info::RdUserInfo,
    },
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
    let claim = generate_claim(&token.user_id, &token.device_id, app);
    let rd_user = RdUserInfo {
        id: claim.userId,
        nickname: "nickname".to_owned(),
        device_id: claim.deviceId,
        app_id: claim.appId,
    };
    let access_token = create_access_token(&rd_user);
    let resp = AuthResp::from(access_token);
    return box_actix_rest_response(resp);
}

fn generate_claim(user_id: &i64, device_id: &String, app: App) -> JwtPayload {
    let jwt_payload = JwtPayload {
        userId: user_id.to_owned(),
        deviceId: device_id.to_owned(),
        appId: app.app_id,
        lt: 0,
        et: 0,
        pid: app.product_id,
    };
    return jwt_payload;
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/infra/auth")
            .route("/refresh-access-token", web::get().to(refresh_access_token)),
    );
}
