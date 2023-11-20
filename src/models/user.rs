use crate::schema::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Eq, Queryable, Identifiable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub tel: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, PartialEq, Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    #[schema(example = "user")]
    pub username: &'a str,
    #[schema(example = "user@ex.com")]
    pub email: &'a str,
    #[schema(example = "111111")]
    pub password: Option<&'a str>,
    #[schema(example = "0999999999")]
    pub tel: Option<&'a str>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    #[schema(example = "user")]
    pub username: String,
    #[schema(example = "user@ex.com")]
    pub email: String,
    #[schema(example = "111111")]
    pub password: Option<String>,
    #[schema(example = "0999999999")]
    pub tel: Option<String>,
}

#[derive(AsChangeset, ToSchema)]
#[diesel(table_name = users)]
pub struct UpdatedUser<'a> {
    #[schema(example = "user")]
    pub username: &'a str,
    #[schema(example = "user@ex.com")]
    pub email: &'a str,
    #[schema(example = "111111")]
    pub password: Option<&'a str>,
    #[schema(example = "0999999999")]
    pub tel: Option<&'a str>,
}
