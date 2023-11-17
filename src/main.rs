mod config;
mod controllers;
mod database;
mod models;
mod routes;
mod schema;
mod services;
mod utils;
mod docs;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;

use config::config::CONFIG;
use database::database::db_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = CONFIG.port.clone().parse().expect("Failed to parse port number");
    let host = CONFIG.host.clone();
    let database_url = CONFIG.database_url.clone();
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let doc = api_doc::create_api_doc();
    println!("{}", doc.to_pretty_json());
    
    HttpServer::new(move || {
        let connection = db_connection(&database_url);

        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(connection.clone()))
            .service(web::scope("/apis").configure(routes::routes::config_routes))
    })
    .bind((host, port))?
    .run()
    .await
}
