use chrono::NaiveDateTime;

#[derive(Debug, PartialEq, DbEnum)]
pub enum UserRole {
    Admin,
    User,
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        tel -> Nullable<Varchar>,
        role -> UserRole,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    shops (id) {
        id -> Integer,
        name -> Varchar,
        address -> Text,
        telephone -> Nullable<Varchar>,
        user_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(shops -> users (user_id));

allow_tables_to_appear_in_same_query!(users, shops);
