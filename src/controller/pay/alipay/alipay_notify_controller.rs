use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use log::warn;
use crate::composite::pay::alipay::alipay_callback_handler::handle_pay_callback;
use crate::model::pay::callback::alipay_callback::AlipayCallback;

/// Recieve notifycation
///
/// Recieve notifycation
#[utoipa::path(
    context_path = "/infra/alipay/pay",
    path = "/",
    responses(
        (status = 200, description = "Recieve notifycation")
    )
)]
#[post("/v1/alipaySeverNotification")]
pub async fn alipay_server_notify(payload: web::Json<AlipayCallback>) -> impl Responder {
    warn!("receive alipay callback, payload: {:?}", payload.0);
    handle_pay_callback(&payload.0);
    return HttpResponse::Unauthorized().finish();
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/alipay/notification").service(alipay_server_notify);
    conf.service(scope);
}
