mod config;
mod controllers;
mod database;
mod models;
mod routes;
mod schema;
mod services;
mod utils;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::controllers::user_controller;
use crate::models::user::{CreateUserRequest, NewUser, User};
use config::config::CONFIG;
use database::database::db_connection;

#[derive(OpenApi)]
#[openapi(info(
    title = "My API",
    version = "1.0",
    description = "My API Documentation"
))]
#[openapi(
    paths(
        user_controller::get_all_users,
        user_controller::get_user_by_id,
        user_controller::create_user,
        user_controller::update_user,
        user_controller::delete_user),
    components(schemas(NewUser, CreateUserRequest, User)),
    tags(
        (name = "users", description = "User management endpoints.")
    ),
)]

pub struct ApisDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = CONFIG
        .port
        .clone()
        .parse()
        .expect("Failed to parse port number");
    let host = CONFIG.host.clone();
    let database_url = CONFIG.database_url.clone();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let openapi = ApisDoc::openapi();

    HttpServer::new(move || {
        let connection = db_connection(&database_url);

        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(connection.clone()))
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
            .service(web::scope("/apis").configure(routes::routes::config_routes))
    })
    .bind((host, port))?
    .run()
    .await
}
