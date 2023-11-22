use crate::schema::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(
    Debug,
    PartialEq,
    Eq,
    Queryable,
    Selectable,
    Identifiable,
    Serialize,
    Deserialize,
    ToSchema,
    Validate,
    Clone,
)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "user")]
    pub username: String,
    #[schema(example = "user@ex.com")]
    #[validate(email)]
    pub email: String,
    pub password: Option<String>,
    #[schema(example = "0999999999")]
    pub tel: Option<String>,
    #[schema(example = "2022-01-01 00:00:00")]
    pub created_at: NaiveDateTime,
    #[schema(example = "2022-01-01 00:00:00")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, PartialEq, Insertable, Serialize, Deserialize, ToSchema, Validate)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    #[schema(example = "user", required = true)]
    #[validate(length(min = 1, max = 255))]
    pub username: &'a str,
    #[schema(example = "user@ex.com", required = true)]
    #[validate(email)]
    pub email: &'a str,
    #[schema(example = "111111", required = false)]
    pub password: Option<&'a str>,
    #[schema(example = "0999999999", required = false)]
    pub tel: Option<&'a str>,
}

#[derive(Debug, Insertable, Serialize, Deserialize, ToSchema, Validate)]
#[diesel(table_name = users)]
pub struct CreateUserRequest {
    #[schema(example = "user")]
    #[validate(length(min = 1, max = 255))]
    pub username: String,
    #[schema(example = "user@ex.com")]
    #[validate(email)]
    pub email: String,
    #[schema(example = "111111")]
    pub password: Option<String>,
    #[schema(example = "0999999999")]
    pub tel: Option<String>,
}

#[derive(AsChangeset, ToSchema, Validate)]
#[diesel(table_name = users)]
pub struct UpdatedUser<'a> {
    #[schema(example = "user")]
    #[validate(length(min = 1, max = 255))]
    pub username: &'a str,
    #[schema(example = "user@ex.com")]
    #[validate(email)]
    pub email: &'a str,
    #[schema(example = "111111")]
    pub password: Option<&'a str>,
    #[schema(example = "0999999999")]
    pub tel: Option<&'a str>,
}
