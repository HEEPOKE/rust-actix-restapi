use chrono::NaiveDateTime;
use diesel::table;

#[derive(Debug, PartialEq, Eq, , DbEnum)]
pub enum UserRole {
    Admin,
    User,
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Nullable<Varchar>,
        tel -> Nullable<Varchar>,
        role -> UserRole,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    shops (id) {
        id -> Int4,
        name -> Varchar,
        address -> Text,
        telephone -> Nullable<Varchar>,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(shops -> users (user_id));

allow_tables_to_appear_in_same_query!(users, shops);
