use crate::controller::utils::snowflake::SnowflakeIdWorker;
use actix_web::{get, web, Responder};
use log::error;
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;

/// Get uniq id
///
/// Uniq id
#[utoipa::path(
    context_path = "/infra/util/uniqid",
    path = "/",
    responses(
        (status = 200, description = "get product list")
    )
)]
#[get("/uniqid/gen")]
pub async fn id_gen() -> impl Responder {
    let id_generator = SnowflakeIdWorker::new(2, 2).unwrap();
    let uniq_id = id_generator.next_id();
    if let Err(e) = uniq_id.as_ref() {
        error!("gen uniq id failed,{}", e);
    }
    return box_actix_rest_response(uniq_id.unwrap());
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra/util").service(id_gen);
    conf.service(scope);
}
