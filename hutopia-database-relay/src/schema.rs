// @generated automatically by Diesel CLI.

diesel::table! {
    relay_google_tokens (id) {
        id -> Int4,
        user_id -> Text,
        access_secret -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    relay_user_tokens (user_id, token) {
        user_id -> Text,
        token -> Text,
    }
}

diesel::table! {
    relay_users (id) {
        id -> Text,
        email -> Text,
        username -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(relay_google_tokens -> relay_users (user_id));
diesel::joinable!(relay_user_tokens -> relay_users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    relay_google_tokens,
    relay_user_tokens,
    relay_users,
);
