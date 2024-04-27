use crate::composite::pay::alipay::pay_impl::do_alipay;
use actix_web::{get, web, Responder};
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;

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
pub async fn create_order() -> impl Responder {
    do_alipay().await;
    return box_actix_rest_response("ok");
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/alipay/pay").service(create_order);
    conf.service(scope);
}
