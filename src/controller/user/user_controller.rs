
use actix_web::{web, Responder};
use rust_wheel::common::wrapper::actix_http_resp::box_actix_rest_response;

#[derive(serde::Deserialize)]
pub struct FileQueryParams {
    pub file_id: String,
}

pub async fn get_file(_params: web::Query<FileQueryParams>) -> impl Responder {
    box_actix_rest_response("ok")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/infra/user")
            .route("/list", web::get().to(get_file)),
    );
}

