// @generated automatically by Diesel CLI.

diesel::table! {
    csrf_tokens (csrf_token) {
        csrf_token -> Text,
    }
}

diesel::table! {
    google_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        access_secret -> Text,
        refresh_secret -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_permissions (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(google_tokens -> users (user_id));
diesel::joinable!(user_permissions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    csrf_tokens,
    google_tokens,
    message,
    user_permissions,
    users,
);
