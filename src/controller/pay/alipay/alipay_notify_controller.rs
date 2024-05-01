use crate::{
    composite::pay::alipay::pay_impl::prepare_pay, model::req::goods::goods_req::GoodsReq,
    service::goods::goods_service::query_goods_by_id,
};
use actix_web::{post, web, Responder};
use rust_wheel::{
    common::wrapper::actix_http_resp::box_actix_rest_response,
    model::user::login_user_info::LoginUserInfo,
};

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
pub async fn alipay_server_notify(
    form: web::Json<GoodsReq>,
    login_user_info: LoginUserInfo,
) -> impl Responder {
    let good = query_goods_by_id(&form.0.productId);
    prepare_pay(&login_user_info, &good);
    return box_actix_rest_response("ok");
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/alipay/notification").service(alipay_server_notify);
    conf.service(scope);
}
