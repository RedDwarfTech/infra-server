use actix_web::{post, web, Responder};
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;
use crate::{model::req::notify::sms::sms_req::SmsReq, service::notify::sms_service::send_sms};

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
    match send_sms(&_params.0) {
        Ok(response) => box_actix_rest_response(response),
        Err(_err) => box_actix_rest_response("err"),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope_inner = web::scope("/infra-inner/sms").service(send);
    conf.service(scope_inner);
}
