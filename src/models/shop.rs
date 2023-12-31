use crate::models::user::User;
use crate::schema::schema::shops;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, PartialEq, Queryable, Selectable, Associations, Identifiable, Serialize, Deserialize,
)]
#[diesel(table_name = shops)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Shop {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub telephone: Option<String>,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = shops)]
pub struct NewShop<'a> {
    pub name: &'a str,
    pub address: &'a str,
    pub telephone: Option<&'a str>,
}
