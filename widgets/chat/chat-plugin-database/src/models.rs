use diesel::prelude::*;

use crate::schema::chat_message;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = chat_message)]
pub struct Message {
    pub username: String,
    pub message_text: String,
}

#[derive(Insertable)]
#[diesel(table_name = chat_message)]
pub struct NewMessage<'a> {
    pub username: &'a str,
    pub message_text: &'a str,
}