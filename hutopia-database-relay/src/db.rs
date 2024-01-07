use diesel::{prelude::*, r2d2::{PooledConnection, Pool, ConnectionManager}};
use diesel_migrations::*;

/*use crate::{
    models::{NewUser, User},
    ops::{
        user_ops as users
    }
};*/

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

        Self { pool }
    }

    fn get_connection(&self) -> PooledConnection<ConnectionManager<diesel::PgConnection>> {
        self.pool.get().unwrap()
    }

    pub fn run_embedded_migrations(&self) {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

        self.get_connection().run_pending_migrations(MIGRATIONS).unwrap();
    }

    /*
    use uuid::Uuid;
    pub fn create_user(&self, uuid: &Uuid) -> bool {
        let new_user = NewUser { id: uuid };

        users::create_user(&mut self.get_connection(), new_user)
    }


    pub fn get_user(&self, username: &str) -> Option<User> {
        users::get_user(&mut self.get_connection(), username)
    }*/

}