// @generated automatically by Diesel CLI.

diesel::table! {
    responses (responder_uuid) {
        survey_id -> Int4,
        responder_uuid -> Uuid,
        content -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    surveys (id) {
        id -> Int4,
        title -> Text,
        description -> Text,
        published -> Bool,
        owner_id -> Int4,
        questions -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(responses -> surveys (survey_id));
diesel::joinable!(surveys -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(responses, surveys, users,);
