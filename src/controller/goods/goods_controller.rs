use crate::service::app::app_service::query_cached_app;
use crate::service::goods::goods_service::query_goods_list;
use actix_web::{get, web, Responder};
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

/// Get product list
///
/// product list
#[utoipa::path(
    context_path = "/infra/product/list",
    path = "/",
    responses(
        (status = 200, description = "get product list")
    )
)]
#[get("/list")]
pub async fn prod_list(login_user_info: LoginUserInfo) -> impl Responder {
    let app = query_cached_app(&login_user_info.appId);
    let goods = query_goods_list(&app.product_id);
    return box_actix_rest_response(goods);    
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/product")
        .service(prod_list);
    conf.service(scope);
}
