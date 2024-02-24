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
    pub plugin_handler: PluginHandler,
    // <token, user>
    pub auth_tokens: Arc<Mutex<HashMap<String, String>>>,
}

impl ServerData {
    pub fn new(config: &Box<SpaceConfig>) -> Self {
        // init db
        //let db_env = config.env.db_connection.clone();
        //let url = std::env::var(db_env).expect("DB Env var to be set.");
        //let db = Database::new(url);

        let relay_uri = config.server.relay.to_string();
        let plugin_handler = ServerData::get_plugin_handler();
        let auth_tokens = Arc::new(Mutex::new(HashMap::new()));

        let server_data = Self {
            /*db,*/
            relay_uri,
            plugin_handler,
            auth_tokens,
        };

        server_data.ensure_dependencies();

        server_data
    }

    /* Auth Functions */

    pub async fn auth_user(&self, username: &str, token: &str) -> bool {
        // Check the cache
        {
            // inner scope to limit the lock time on the tokens
            let mut map = self.auth_tokens.lock().unwrap();
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

    /* Plugin functions */

    fn get_plugin_handler() -> PluginHandler {
        let mut plugin_handler = PluginHandler::new();

        if let Ok(entries) = std::fs::read_dir(PLUGINS_FOLDER) {
            for entry in entries.flatten() {
                if let Ok(file_path) = entry.path().into_os_string().into_string() {
                    if file_path.ends_with(LIB_EXTENSION) {
                        unsafe {
                            plugin_handler
                                .load(file_path)
                                .expect("Plugin loading failed");
                        }
                    }
                }
            }
        }

        plugin_handler
    }

    fn ensure_dependencies(&self) {
        for (plugin_name, plugin_proxy) in self.plugin_handler.plugins.iter() {
            let dependencies = plugin_proxy.get_plugin_dependencies();

            for dependency in dependencies {
                if !self.plugin_handler.plugins.contains_key(&dependency) {
                    let msg = format!("Dependency \"{}\" for plugin \"{}\" is missing!", dependency, plugin_name);
                    
                    log::error!("{msg}");
                    panic!("{msg}");
                }
            }
        }
    }
}
