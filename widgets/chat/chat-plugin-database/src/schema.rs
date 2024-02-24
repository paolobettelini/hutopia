// @generated automatically by Diesel CLI.

diesel::table! {
    chat_message (id) {
        id -> Int4,
        username -> Text,
        message_text -> Text,
    }
}