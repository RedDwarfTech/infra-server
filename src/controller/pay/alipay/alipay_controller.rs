use crate::{composite::pay::alipay::pay_impl::{do_alipay, prepare_pay}, controller::user::user_controller::login, model::req::goods::goods_req::GoodsReq, service::goods::goods_service::query_goods_by_id};
use actix_web::{get, web, Responder};
use rust_wheel::{common::wrapper::actix_http_resp::box_actix_rest_response, model::user::login_user_info::LoginUserInfo};

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
pub async fn create_order(params: web::Query<GoodsReq>,login_user_info: LoginUserInfo) -> impl Responder {
    let good = query_goods_by_id(&params.0.productId);
    prepare_pay(&login_user_info, &good);
    return box_actix_rest_response("ok");
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/alipay/pay").service(create_order);
    conf.service(scope);
}
