use crate::model::resp::goods::goods_resp::GoodsResp;
use crate::service::app::app_service::query_cached_app;
use crate::service::goods::goods_service::query_goods_list;
use actix_web::{get, web, Responder};
use rust_wheel::common::util::model_convert::map_entity;
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

/// Get product list
///
/// product list
#[utoipa::path(
    context_path = "/infra/util/uniqid",
    path = "/",
    responses(
        (status = 200, description = "get product list")
    )
)]
#[get("/uniqid/gen")]
pub async fn prod_list(login_user_info: LoginUserInfo) -> impl Responder {
    let app = query_cached_app(&login_user_info.appId);
    let goods = query_goods_list(&app.product_id);
    let resp_goods: Vec<GoodsResp> =  map_entity(goods);
    return box_actix_rest_response(resp_goods);    
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/util")
        .service(prod_list);
    conf.service(scope);
}
