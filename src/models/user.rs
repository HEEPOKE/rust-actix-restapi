use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, PartialEq, DbEnum)]
pub enum UserRole {
    Admin,
    User,
}
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

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: Option<&'a str>,
    pub tel: Option<&'a str>,
    pub role: &'a UserRole,
}
