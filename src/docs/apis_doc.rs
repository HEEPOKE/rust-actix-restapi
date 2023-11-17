use utoipa::OpenApi;
use crate::models::user::User;
use crate::controllers::user_controller;

#[derive(OpenApi)]
#[openapi(
    info(description = "My API Documentation"),
)]
#[openapi(paths(user_controller::get_all_users), components(schemas(User)))]
pub struct ApisDoc;

pub fn create_api_doc() -> ApisDoc {
    let mut doc = ApisDoc::openapi();
    doc.info.title = String::from("My API");
    doc
}