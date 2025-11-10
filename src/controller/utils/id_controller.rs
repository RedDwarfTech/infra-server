use crate::controller::utils::snowflake::SnowflakeIdWorker;
use actix_web::{get, web, Responder};
use log::error;
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;

/// Get uniq id
///
/// Uniq id
#[utoipa::path(
    context_path = "/infra-inner/util/uniqid/gen",
    path = "/",
    responses(
        (status = 200, description = "get uniq id")
    )
)]
#[get("/uniqid/gen")]
pub async fn id_gen() -> impl Responder {
    // parse the worker_id and data_center_id from kubernetes statefulset pod name
    let statefulset_service_name = get_app_config("infra.infra_service_name");
    let hostname = std::env::var("HOSTNAME").unwrap_or_default();
    
    // Extract index from hostname (format: {statefulset-name}-{index})
    let index = hostname.split('-')
        .last()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    
    // Ensure index is within valid range (0-1023)
    let index = index.min(1023);
    
    // Extract worker_id (first 5 bits) and datacenter_id (last 5 bits)
    let worker_id = (index & 31) as i64;  // 31 = 0b11111
    let datacenter_id = ((index >> 5) & 31) as i64;
    
    let id_generator = SnowflakeIdWorker::new(worker_id, datacenter_id).unwrap();
    let uniq_id = id_generator.next_id();
    if let Err(e) = uniq_id.as_ref() {
        error!("gen uniq id failed,{}", e);
    }
    return box_actix_rest_response(uniq_id.unwrap());
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/infra-inner/util").service(id_gen);
    conf.service(scope);
}