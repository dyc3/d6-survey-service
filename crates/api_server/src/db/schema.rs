// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}