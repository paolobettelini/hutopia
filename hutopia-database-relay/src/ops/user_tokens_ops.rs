use crate::models::*;
use diesel::prelude::*;

pub fn add_user_token(connection: &mut PgConnection, new_user_token: NewUserToken) -> bool {
    use crate::schema::user_tokens::dsl::*;

    diesel::insert_into(user_tokens)
        .values(&new_user_token)
        .execute(connection)
        .is_ok()
}

pub fn get_user_tokens(connection: &mut PgConnection, user_id: &str) -> Option<Vec<String>> {
    use crate::schema::user_tokens::dsl::*;

    user_tokens
        .filter(user_id.eq(user_id))
        .select(token)
        .load::<String>(connection)
        .ok()
}

pub fn user_has_token(connection: &mut PgConnection, user_id: &str, token: &str) -> bool {
    use crate::schema::user_tokens::dsl::*;

    user_tokens
        .filter(user_id.eq(user_id).and(token.eq(token)))
        .first::<UserToken>(connection)
        .is_ok()
}