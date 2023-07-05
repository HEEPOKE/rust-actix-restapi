use actix_web::{web, App, HttpServer};

mod config;

use config::config::CONFIG;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = CONFIG.port.parse().expect("Failed to parse port number");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
