use crate::auth::g_auth::UnregisteredUser;
use crate::RelayConfig;
use hutopia_database_relay::db::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub(crate) struct ServerData {
    /// Database pool
    pub db: Database,
    /// Auth config
    pub auth: GoogleAuthConfig,
    /// Users who are about to create their accounts in the /register page
    /// <Session token, Unregistered User>
    // TODO: keep for only 1 hour
    pub unregistered_users: Arc<Mutex<HashMap<String, UnregisteredUser>>>,
    /// Tokens used by the space to authenticate a user to its server.
    /// The space will ask this server for the token to see if it matches
    /// the one provided by the user.
    /// <token, username>
    // TODO: keep for only 5 minutes
    pub space_auth_tokens: Arc<Mutex<HashMap<String, String>>>,
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

        // Caches
        let unregistered_users = Arc::new(Mutex::new(HashMap::new()));
        let space_auth_tokens = Arc::new(Mutex::new(HashMap::new()));

        Self {
            db,
            auth,
            unregistered_users,
            space_auth_tokens,
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

    pub fn add_space_auth_token(&self, username: String, token: String) {
        log::debug!("Adding unregistered space auth token to RAM {token}");

        let mut map = self.space_auth_tokens.lock().unwrap();
        map.insert(token, username);
    }

    pub fn take_space_auth_tokens(&self, username: &str, token: &str) -> bool {
        let mut map = self.space_auth_tokens.lock().unwrap();
        
        if let Some(user) = map.remove(token) {
            user == username
        } else {
            false
        }
    }
}
