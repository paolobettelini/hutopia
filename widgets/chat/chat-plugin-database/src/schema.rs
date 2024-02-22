// @generated automatically by Diesel CLI.

diesel::table! {
    chat_message (id) {
        id -> Int4,
        user_id -> Uuid,
        message_text -> Text,
    }
}