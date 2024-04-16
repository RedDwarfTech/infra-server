use crate::{model::user::auth::access_token_refresh_req::AccessTokenRefreshReq, service::oauth::oauth_service::query_refresh_token};
use actix_web::{web, Responder};
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;
use sha256::{digest};

pub async fn refresh_access_token(
    form: actix_web_validator::Json<AccessTokenRefreshReq>,
) -> impl Responder {
    let input = String::from(&form.0.refresh_token);
    let val = digest(input);
    let token = query_refresh_token(&form.0.refresh_token);
    
    return box_actix_rest_response("");
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/infra/auth")
            .route("/refresh-access-token", web::get().to(refresh_access_token)),
    );
}
