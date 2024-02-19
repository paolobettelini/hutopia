use crate::SpaceConfig;
use hutopia_database_space::db::*;

#[derive(Debug, Clone)]
pub(crate) struct ServerData {
    /// Database pool
    pub db: Database,
}

impl ServerData {
    pub fn new(config: &Box<SpaceConfig>) -> Self {
        // init db
        let db_env = config.env.db_connection.clone();
        let url = std::env::var(db_env).expect("DB Env var to be set.");
        let db = Database::new(url);

        Self { db }
    }
}
