use diesel::{prelude::*, r2d2::{PooledConnection, Pool, ConnectionManager}};
use diesel_migrations::*;

use crate::{
    models::*,
    /*ops::{
        user_ops as users,
        user_tokens_ops as tokens,
    }*/
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

}