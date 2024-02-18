use hutopia_database_relay::db::*;
use crate::RelayConfig;

#[derive(Debug, Clone)]
pub(crate) struct ServerData {
    pub db: Database,
    pub config: AuthConfig,
}

#[derive(Debug, Clone)]
pub(crate) struct AuthConfig {
    pub redirect_url: String,
    pub client_secret: String,
    pub client_id: String,
}

impl ServerData {
    pub fn new(config: &Box<RelayConfig>) -> Self {
        // init db
        let db_env = config.env.db_connection.clone();
        let url = std::env::var(db_env)
            .expect("DB Env var to be set.");
        let db = Database::new(url);

        // read auth env
        let redirect_url = std::env::var(&config.env.redirect_url)
            .expect("REDIRECT_URL Env var to be set.");
        let client_secret = std::env::var(&config.env.g_auth_secret)
            .expect("G_AUTH_SECRET Env var to be set.");
        let client_id = std::env::var(&config.env.g_auth_client_id)
            .expect("G_AUTH_CLIENT_ID Env var to be set.");

        let config = AuthConfig {
            redirect_url,
            client_secret,
            client_id,
        };
        
        Self { db, config }
    }
}