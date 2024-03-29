use diesel::prelude::*;
use crate::schema::{relay_users, relay_user_tokens};
use chrono::NaiveDateTime;

#[derive(Insertable)]
#[diesel(table_name = relay_users)]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub email: &'a str,
    pub username: &'a str,
}

#[derive(Queryable, Debug)]
#[diesel(table_name = relay_users)]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = relay_user_tokens)]
pub struct NewUserToken<'a> {
    pub user_id: &'a str,
    pub token: &'a str,
}

#[derive(Queryable, Debug)]
#[diesel(table_name = relay_user_tokens)]
pub struct UserToken {
    pub user_id: String,
    pub token: String,
}