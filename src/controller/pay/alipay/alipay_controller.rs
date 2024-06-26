use crate::{
    composite::pay::alipay::pay_impl::prepare_pay, model::req::goods::goods_req::GoodsReq,
    service::goods::goods_service::query_goods_by_id,
};
use actix_web::{ post, web, Responder};
use rust_wheel::{
    common::wrapper::actix_http_resp::box_actix_rest_response,
    model::user::login_user_info::LoginUserInfo,
};

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
#[post("/createOrder")]
pub async fn create_order(
    form: web::Json<GoodsReq>,
    login_user_info: LoginUserInfo,
) -> impl Responder {
    let good = query_goods_by_id(&form.0.productId);
    let order_resp = prepare_pay(&login_user_info, &good);
    return box_actix_rest_response(order_resp);
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/alipay/pay").service(create_order);
    conf.service(scope);
}
