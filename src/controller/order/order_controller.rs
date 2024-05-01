use crate::model::req::order::order_status_req::OrderStatusReq;
use crate::model::resp::order::order_status_resp::OrderStatusResp;
use crate::service::order::order_service::{query_order_by_order_id, query_order_by_user_id};
use actix_web::{get, web, Responder};
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

/// Get order status
///
/// order status
#[utoipa::path(
    context_path = "/infra/order/status",
    path = "/",
    responses(
        (status = 200, description = "get order status")
    )
)]
#[get("/status")]
pub async fn get_order_status(
    params: web::Query<OrderStatusReq>,
    login_user_info: LoginUserInfo,
) -> impl Responder {
    let db_order = query_order_by_order_id(&params.orderId, &login_user_info.userId);
    let status_resp = OrderStatusResp::from(&db_order);
    return box_actix_rest_response(status_resp);
}

/// Get order status
///
/// order status
#[utoipa::path(
    context_path = "/infra/order/status",
    path = "/",
    responses(
        (status = 200, description = "get order status")
    )
)]
#[get("/list")]
pub async fn get_user_orders(
    login_user_info: LoginUserInfo,
) -> impl Responder {
    let db_order = query_order_by_user_id(&login_user_info.userId);
    return box_actix_rest_response(db_order);
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/order")
    .service(get_order_status)
    .service(get_user_orders);
    conf.service(scope);
}
