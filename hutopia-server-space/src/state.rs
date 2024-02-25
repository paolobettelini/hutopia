use crate::{PluginHandler, SpaceConfig, LIB_EXTENSION, PLUGINS_FOLDER};
use hutopia_database_space::db::*;
use reqwest::Client;
use serde_json::Value;
use hutopia_plugin_server::IPlugin;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub(crate) struct ServerData {
    /// Database pool
    // pub db: Database,
    /// Relay URI
    pub relay_uri: String,
    // <token, user>
    // TODO: keep for only 1 day
    pub auth_tokens: Arc<Mutex<HashMap<String, String>>>,
}

impl ServerData {
    pub fn new(config: &Box<SpaceConfig>) -> Self {
        // init db
        //let db_env = config.env.db_connection.clone();
        //let url = std::env::var(db_env).expect("DB Env var to be set.");
        //let db = Database::new(url);

        let relay_uri = config.server.relay.to_string();
        let auth_tokens = Arc::new(Mutex::new(HashMap::new()));

        let server_data = Self {
            /*db,*/
            relay_uri,
            auth_tokens,
        };

        server_data
    }

    /* Auth Functions */

    pub async fn auth_user(&self, username: &str, token: &str) -> bool {
        // Check the cache
        {
            // inner scope to limit the lock time on the tokens
            let map = self.auth_tokens.lock().unwrap();
            if let Some(user) = map.get(token) {
                return user == username;
            }
        }

        // Send the query to relay otherwise

        // When we do this query, the token is consumated and removed from the relay,
        // so it's important to cache this information.
        let url = format!(
            "{}/api/checkSpaceAuthToken/{}/{}",
            self.relay_uri, username, token
        );

        let client = Client::new();
        let response = if let Ok(v) = client.post(&url).send().await {
            v
        } else {
            return false;
        };

        let json: Value = if let Ok(v) = response.json().await {
            v
        } else {
            return false;
        };

        if let Some(authenticated) = json.get("authenticated").and_then(|v| v.as_bool()) {
            if authenticated {
                // Add to cache
                let mut map = self.auth_tokens.lock().unwrap();
                map.insert(token.to_string(), username.to_string());
            }
            
            authenticated
        } else {
            false
        }
    }

}
