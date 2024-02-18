use crate::auth::g_auth::UnregisteredUser;
use crate::RelayConfig;
use hutopia_database_relay::db::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// TODO: a system to empty unregistered_users sometimes
// by keeping a time limit

#[derive(Debug, Clone)]
pub(crate) struct ServerData {
    /// Database pool
    pub db: Database,
    /// <Session token, Unregistered User>
    pub unregistered_users: Arc<Mutex<HashMap<String, UnregisteredUser>>>,
    /// Auth config
    pub auth: GoogleAuthConfig,
}

#[derive(Debug, Clone)]
pub(crate) struct GoogleAuthConfig {
    pub redirect_url: String,
    pub client_secret: String,
    pub client_id: String,
}

impl ServerData {
    pub fn new(config: &Box<RelayConfig>) -> Self {
        // init db
        let db_env = config.env.db_connection.clone();
        let url = std::env::var(db_env).expect("DB Env var to be set.");
        let db = Database::new(url);

        // read auth env
        let redirect_url =
            std::env::var(&config.env.redirect_url).expect("REDIRECT_URL Env var to be set.");
        let client_secret =
            std::env::var(&config.env.g_auth_secret).expect("G_AUTH_SECRET Env var to be set.");
        let client_id = std::env::var(&config.env.g_auth_client_id)
            .expect("G_AUTH_CLIENT_ID Env var to be set.");

        // auth config
        let auth = GoogleAuthConfig {
            redirect_url,
            client_secret,
            client_id,
        };

        // Unregistered users
        let unregistered_users = Arc::new(Mutex::new(HashMap::new()));

        Self {
            db,
            unregistered_users,
            auth,
        }
    }

    pub fn add_unregistered_user(&self, token: String, user: UnregisteredUser) {
        log::debug!("Adding unregistered user to RAM {token}");

        let mut map = self.unregistered_users.lock().unwrap();
        map.insert(token, user);
    }

    pub fn take_unregistered_user(&self, token: String) -> Option<UnregisteredUser> {
        let mut map = self.unregistered_users.lock().unwrap();
        map.remove(&token)
    }
}
