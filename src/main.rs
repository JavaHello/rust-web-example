#[macro_use]
extern crate lazy_static;
extern crate rbatis;

pub mod config;
pub mod controller;
pub mod core;
pub mod dao;
pub mod mid;
pub mod model;
pub mod service;
pub mod util;

use crate::config::BOOT_CONFIG;
use crate::controller::{user_controller, verify_code_controller};
use actix_web::middleware;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
// use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
// use rbatis::rbatis::Rbatis;
// use serde_json::json;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //初始化日志
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    //初始化路由，启动http服务
    HttpServer::new(|| {
        let auth = HttpAuthentication::bearer(mid::validator);
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/v1")
                    .wrap(auth)
                    .configure(controller::configure),
            )
            .route("/login", web::post().to(user_controller::user_login))
            .route("/register", web::post().to(user_controller::user_reg))
            .route(
                "/verify_code",
                web::post().to(verify_code_controller::verify_code),
            )
            .route("/", web::get().to(index))
            .route("/index", web::get().to(index))
    })
    .bind(&BOOT_CONFIG.server_url)?
    .run()
    .await
}
