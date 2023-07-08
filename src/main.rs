mod config;
mod controllers;
mod database;
mod models;
mod routes;
mod schema;
mod services;

use actix_web::{web, App, HttpServer};

use config::config::CONFIG;
use database::database::db_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = CONFIG.port.parse().expect("Failed to parse port number");

    let connection = db_connection(&CONFIG.database_url);

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(connection.clone()))
            .service(routes::user_route())
    })
    .bind((CONFIG.host, port))?
    .run()
    .await
}
