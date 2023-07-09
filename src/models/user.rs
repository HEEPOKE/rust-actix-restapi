use crate::schema::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "users"]
#[primary_key(id)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub tel: Option<String>,
}

#[derive(Debug, PartialEq, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: Option<&'a str>,
    pub tel: Option<&'a str>,
}
