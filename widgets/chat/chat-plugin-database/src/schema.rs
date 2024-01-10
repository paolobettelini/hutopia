// @generated automatically by Diesel CLI.

diesel::table! {
    message (id) {
        id -> Int4,
        user_id -> Uuid,
        message_text -> Text,
    }
}
