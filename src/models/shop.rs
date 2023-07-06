use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::schema::shops;

#[derive(Debug, PartialEq, Queryable)]
pub struct Shop {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub telephone: Option<String>,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
