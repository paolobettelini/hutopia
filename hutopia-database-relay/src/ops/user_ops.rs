use crate::models::*;
use diesel::prelude::*;

pub fn create_user(connection: &mut PgConnection, new_user: NewUser) -> bool {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users)
        .values(&new_user)
        .execute(connection)
        .is_ok()
}

pub fn get_user(connection: &mut PgConnection, user_id: &str) -> Option<User> {
    use crate::schema::users::dsl::*;

    users
        .filter(id.eq(user_id))
        .first::<User>(connection)
        .ok()
}

pub fn user_id_exists(connection: &mut PgConnection, user_id: &str) -> bool {
    use crate::schema::users::dsl::*;
    use diesel::{select, dsl::exists};
    
    let result = select(exists(users.filter(id.eq(user_id))))
        .get_result(connection);

    if let Ok(res) = result {
        res
    } else {
        false
    }
}

pub fn user_username_exists(connection: &mut PgConnection, user_username: &str) -> bool {
    use crate::schema::users::dsl::*;
    use diesel::{select, dsl::exists};
    
    let result = select(exists(users.filter(username.eq(user_username))))
        .get_result(connection);

    if let Ok(res) = result {
        res
    } else {
        false
    }
}