mod config;
mod controllers;
mod database;
mod models;
mod routes;
mod schema;
mod services;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;

use config::config::CONFIG;
use database::database::db_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = CONFIG.port.parse().expect("Failed to parse port number");

    let connection = db_connection(&CONFIG.database_url);

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(connection.clone()))
            .service(web::scope("/apis").configure(routes::routes::config_routes))
    })
    .bind((CONFIG.host, port))?
    .run()
    .await
}
