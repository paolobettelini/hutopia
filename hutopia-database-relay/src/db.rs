use diesel::{prelude::*, r2d2::{PooledConnection, Pool, ConnectionManager}};
use diesel_migrations::*;

use crate::{
    models::*,
    ops::{
        user_ops as users,
        user_tokens_ops as tokens,
    }
};

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone, Debug)]
pub struct Database {
    pool: DbPool,
}

impl Database {

    pub fn new(url: String) -> Self {
        let pool = DbPool::builder()
            .max_size(15)
            .build(ConnectionManager::new(url))
            .unwrap();

        Self { pool }
    }

    fn get_connection(&self) -> PooledConnection<ConnectionManager<diesel::PgConnection>> {
        self.pool.get().unwrap()
    }

    pub fn run_embedded_migrations(&self) {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

        self.get_connection().run_pending_migrations(MIGRATIONS).unwrap();
    }

    pub fn create_user(&self, id: &str, email: &str, username: &str) -> bool {
        let new_user = NewUser { id, email, username };
        
        users::create_user(&mut self.get_connection(), new_user)
    }

    pub fn user_id_exists(&self, id: &str) -> bool {
        users::user_id_exists(&mut self.get_connection(), id)
    }

    pub fn username_exists(&self, username: &str) -> bool {
        users::user_username_exists(&mut self.get_connection(), username)
    }

    pub fn get_user_by_id(&self, id: &str) -> Option<User> {
        users::get_user_by_id(&mut self.get_connection(), id)
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<User> {
        users::get_user_by_username(&mut self.get_connection(), username)
    }

    pub fn add_user_token(&self, user_id: &str, token: &str) -> bool {
        let new_user_token = NewUserToken { user_id, token };

        tokens::add_user_token(&mut self.get_connection(), new_user_token)
    }
    
    pub fn get_user_tokens(&self, user_id: &str) -> Option<Vec<String>> {
        tokens::get_user_tokens(&mut self.get_connection(), user_id)
    }

    pub fn user_has_token(&self, user_id: &str, token: &str) -> bool {
        tokens::user_has_token(&mut self.get_connection(), user_id, token)
    }

}