use actix_web::{post, web, Responder};
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;

use crate::model::diesel::custom::system_log::system_log_add::SystemLogAdd;
use crate::model::req::system_log::system_log_req::SystemLogReq;
use crate::service::monitor::system_log_service::save_system_log_with_now;

/// Save system log
///
/// Save system runtime log
#[utoipa::path(
    context_path = "/infra-inner/log",
    path = "/save",
    responses(
        (status = 200, description = "save system log")
    )
)]
#[post("/save")]
pub async fn save(json: web::Json<SystemLogReq>) -> impl Responder {
    let req = json.into_inner();
    let mut log = SystemLogAdd {
        log_time: req.log_time.unwrap_or_default(),
        content: req.content,
        source: req.source,
        level: req.level,
        app_id: req.app_id,
        created_time: 0,
    };
    save_system_log_with_now(&mut log);
    box_actix_rest_response("")
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope_inner = web::scope("/infra-inner/log").service(save);
    conf.service(scope_inner);
}