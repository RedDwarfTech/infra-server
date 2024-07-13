use crate::{model::req::notify::sms::sms_req::SmsReq, service::notify::sms_service::send_sms};
use actix_web::{post, web, Responder};
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;

/// Send sms message
///
/// send sms message
#[utoipa::path(
    context_path = "/infra-inner/sms/send",
    path = "/",
    responses(
        (status = 200, description = "send sms status")
    )
)]
#[post("/send")]
pub async fn send(_params: web::Query<SmsReq>) -> impl Responder {
    // the sms quota of each app check
    let result = send_sms(&_params.0);
    box_actix_rest_response(result.unwrap_or_default())
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope_inner = web::scope("/infra-inner/sms").service(send);
    conf.service(scope_inner);
}
