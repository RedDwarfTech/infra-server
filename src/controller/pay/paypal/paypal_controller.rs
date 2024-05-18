use crate::{
    composite::pay::paypal::paypal_impl::do_paypal_pay, model::req::goods::goods_req::GoodsReq,
    service::goods::goods_service::query_goods_by_id,
};
use actix_web::{post, web, Responder};
use rust_wheel::{
    common::wrapper::actix_http_resp::box_actix_rest_response,
    model::user::login_user_info::LoginUserInfo,
};

/// Create order
///
/// create order
#[utoipa::path(
    context_path = "/infra/paypal/pay",
    path = "/",
    responses(
        (status = 200, description = "create order")
    )
)]
#[post("/createOrder")]
pub async fn create_order(
    form: web::Json<GoodsReq>,
    login_user_info: LoginUserInfo,
) -> impl Responder {
    let good = query_goods_by_id(&form.0.productId);
    do_paypal_pay(&login_user_info, &good).await;
    return box_actix_rest_response("ok");
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/paypal/pay").service(create_order);
    conf.service(scope);
}
