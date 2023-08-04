mod config;
mod controllers;
mod database;
mod models;
mod routes;
mod schema;
mod services;

use actix_web::{web, web::scope,middleware::Logger, App, HttpServer};
use log::info;

use config::config::CONFIG;
use database::database::db_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = CONFIG.port.parse().expect("Failed to parse port number");

    let connection = db_connection(&CONFIG.database_url);

    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();
        
        App::new()
            .app_data(web::Data::new(connection.clone()))
            .service(scope("/apis").service(routes::user_route::user_routes()))
    })
    .bind((CONFIG.host, port))?
    .run()
    .await
}
