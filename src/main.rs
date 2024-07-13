extern crate openssl;

#[macro_use]
extern crate diesel;

pub mod common;
pub mod composite;
pub mod controller;
pub mod model;
pub mod service;
mod swagger_docs;
mod types;

use std::env;

use crate::controller::user::user_controller;
use crate::swagger_docs::ApiDoc;
use actix_web::App;
use actix_web::HttpServer;
use controller::goods::goods_controller;
use controller::monitor::health_controller;
use controller::order::order_controller;
use controller::pay::alipay::alipay_controller;
use controller::pay::alipay::alipay_notify_controller;
use controller::pay::paypal::paypal_controller;
use controller::user::auth_controller;
use lazy_static::lazy_static;
use rust_wheel::config::app::app_conf_reader::get_app_config;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

lazy_static! {
    static ref VEC: Vec<String> = {
        let ignore_url: String = env::var("IGNORE_LOGIN_URL").expect("ignore url config missing");
        let parts: Vec<String> = ignore_url.split(',').map(|s| s.to_string()).collect();
        parts
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
            .configure(paypal_controller::config)
            .configure(alipay_notify_controller::config)
            .configure(order_controller::config)
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
