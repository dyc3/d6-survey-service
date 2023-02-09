// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
