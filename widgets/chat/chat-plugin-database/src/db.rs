use diesel::{prelude::*, r2d2::{PooledConnection, Pool, ConnectionManager}};
use diesel_migrations::*;
use uuid::Uuid;

use crate::{
    models::{NewMessage, Message},
    ops::{
        message_ops as messages
    }
};

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DbPool,
}

impl Database {

    pub fn new(url: String) -> Self {
        let pool = DbPool::builder()
            .max_size(15)
            .build(ConnectionManager::new(url))
            .unwrap();

        let db = Self { pool };
        db.run_embedded_migrations();
        db
    }

    fn get_connection(&self) -> PooledConnection<ConnectionManager<diesel::PgConnection>> {
        self.pool.get().unwrap()
    }

    pub fn run_embedded_migrations(&self) {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

        self.get_connection().run_pending_migrations(MIGRATIONS).unwrap();
    }

    pub fn insert_message(&self, user_id: &Uuid, message_text: String) -> bool {
        let new_msg = NewMessage { user_id, message_text };

        messages::add_message(&mut self.get_connection(), new_msg)
    }


    pub fn get_messages(&self) -> Vec<Message> {
        messages::get_messages(&mut self.get_connection())
    }

}