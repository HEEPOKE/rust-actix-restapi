use actix_web::{web, App, HttpServer};

mod config;
mod database;

use config::config::CONFIG;
use database::DbPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = CONFIG.port.parse().expect("Failed to parse port number");
    let database_url = &CONFIG.database_url;

    let pool: DbPool = db_connection(database_url);

    HttpServer::new(|| {
        App::new()
            .data(pool.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((CONFIG.host, port))?
    .run()
    .await
}
