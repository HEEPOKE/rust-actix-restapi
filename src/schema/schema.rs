// @generated automatically by Diesel CLI.

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
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Nullable<Varchar>,
        tel -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    shops,
    users,
);
