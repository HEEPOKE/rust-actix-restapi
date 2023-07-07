// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
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

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

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

diesel::joinable!(shops -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    shops,
    users,
);
