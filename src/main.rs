#[macro_use]
extern crate diesel;

mod swagger_docs;
mod types;
pub mod controller;
pub mod model;
pub mod service;
pub mod common;
pub mod composite;

use crate::controller::user::user_controller;
use actix_web::App;
use actix_web::HttpServer;
use controller::monitor::health_controller;
use rust_wheel::config::app::app_conf_reader::get_app_config;
use utoipa_swagger_ui::SwaggerUi;
use crate::swagger_docs::ApiDoc;
use utoipa::OpenApi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = get_app_config("infra.port").parse().unwrap();
    let address = ("0.0.0.0", port);
    HttpServer::new(|| {
        App::new()
            .configure(user_controller::config)
            .configure(health_controller::config)
            .service(
                SwaggerUi::new("/docs-v1/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    })
    .workers(3)
    .bind(address)?
    .run()
    .await
}


