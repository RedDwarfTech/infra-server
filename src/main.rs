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
use controller::goods::goods_controller;
use controller::monitor::health_controller;
use controller::pay::alipay::alipay_controller;
use controller::user::auth_controller;
use rust_wheel::config::app::app_conf_reader::get_app_config;
use utoipa_swagger_ui::SwaggerUi;
use crate::swagger_docs::ApiDoc;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref HASHMAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("/texpub/user/login", "foo");
        m.insert("/texpub/user/reg", "bar");
        m.insert("/texpub/auth/access-token/refresh", "baz");
        m
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let port: u16 = get_app_config("infra.port").parse().unwrap();
    let address = ("0.0.0.0", port);
    HttpServer::new(|| {
        App::new()
            .configure(user_controller::config)
            .configure(health_controller::config)
            .configure(auth_controller::config)
            .configure(goods_controller::config)
            .configure(alipay_controller::config)
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