#[macro_use]
extern crate diesel;

use crate::controller::user::user_controller;
use actix_web::App;
use actix_web::HttpServer;
use controller::monitor::health_controller;
use rust_wheel::config::app::app_conf_reader::get_app_config;

pub mod controller;
pub mod model;
pub mod service;
pub mod common;
pub mod composite;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = get_app_config("infra.port").parse().unwrap();
    let address = ("0.0.0.0", port);
    HttpServer::new(|| {
        App::new()
            .configure(user_controller::config)
            .configure(health_controller::config)
    })
    .workers(3)
    .bind(address)?
    .run()
    .await
}


