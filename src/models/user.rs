use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::schema::users;

#[derive(Debug, PartialEq, Queryable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub tel: Option<String>,
    pub role: UserRole,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, PartialEq, DbEnum)]
pub enum UserRole {
    Admin,
    User,
}
