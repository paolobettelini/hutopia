use diesel::prelude::*;

use uuid::Uuid;
use crate::schema::chat_message;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = chat_message)]
pub struct Message {
    pub user_id: Uuid,
    pub message_text: String,
}

#[derive(Insertable)]
#[diesel(table_name = chat_message)]
pub struct NewMessage<'a> {
    pub user_id: &'a Uuid,
    pub message_text: String,
}