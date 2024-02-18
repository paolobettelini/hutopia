use diesel::prelude::*;
use crate::schema::users;
use chrono::NaiveDateTime;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub email: &'a str,
}

#[derive(Queryable, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub email: String,
    pub created_at: Option<NaiveDateTime>,
}