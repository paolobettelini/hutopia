// @generated automatically by Diesel CLI.

diesel::table! {
    google_tokens (id) {
        id -> Int4,
        user_id -> Text,
        access_secret -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        email -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(google_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    google_tokens,
    users,
);
