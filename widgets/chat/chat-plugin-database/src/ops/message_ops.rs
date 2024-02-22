use crate::models::*;
use diesel::prelude::*;
use uuid::Uuid;

pub fn add_message(connection: &mut PgConnection, new_message: NewMessage) -> bool {
    use crate::schema::chat_message::dsl::*;

    diesel::insert_into(chat_message)
        .values(&new_message)
        .execute(connection)
        .is_ok()
}

pub fn get_messages(connection: &mut PgConnection) -> Vec<Message> {
    use crate::schema::chat_message::dsl::*;

    chat_message
        .select(Message::as_select())
        .load::<Message>(connection)
        .unwrap()
}