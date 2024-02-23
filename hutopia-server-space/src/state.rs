use crate::{PluginHandler, SpaceConfig, LIB_EXTENSION, PLUGINS_FOLDER};
use hutopia_database_space::db::*;
use reqwest::Client;
use serde_json::Value;

pub(crate) struct ServerData {
    /// Database pool
    // pub db: Database,
    /// Relay URI
    pub relay_uri: String,
    pub plugin_handler: PluginHandler,
}

impl ServerData {
    pub fn new(config: &Box<SpaceConfig>) -> Self {
        // init db
        let db_env = config.env.db_connection.clone();
        let url = std::env::var(db_env).expect("DB Env var to be set.");
        //let db = Database::new(url);

        let relay_uri = config.server.relay.to_string();
        let plugin_handler = ServerData::get_plugin_handler();

        Self {
            /*db,*/ relay_uri,
            plugin_handler,
        }
    }

    pub async fn auth_user(&self, username: &str, token: &str) -> bool {
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
            authenticated
        } else {
            false
        }
    }

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
}
