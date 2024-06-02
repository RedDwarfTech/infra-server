#[macro_use]
extern crate diesel;

pub mod common;
pub mod composite;
pub mod controller;
pub mod model;
pub mod service;
mod swagger_docs;
mod types;

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
use std::collections::HashMap;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

lazy_static! {
    static ref HASHMAP: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(
            "/infra/alipay/notification/v1/alipaySeverNotification",
            "foo",
        );
        m.insert("/infra/auth/access-token/refresh", "foo");
        m.insert("/infra/user/reg", "foo");
        m.insert("/infra/user/pwd/send-verify-code", "foo");
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
