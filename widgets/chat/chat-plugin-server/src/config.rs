use serde::Deserialize;
use std::fs;
use hutopia_utils::config::parse_toml_config;
use std::io::Write;

static CONFIG_FILE: &'static str = "chat.toml";

#[derive(Deserialize, Debug)]
pub struct PluginConfig {
    pub plugin: Plugin,
}

#[derive(Deserialize, Debug)]
pub struct Plugin {
    #[serde(rename(deserialize = "db-connection-env"))]
    pub db_connection_env: String,
}

pub fn get_config() -> Box<PluginConfig> {
    let file = &format!("plugins/{}", CONFIG_FILE);
    
    // Create config file if it doesn't exist
    if !file_exists(file) {
        let mut file = fs::File::create(file).unwrap(); // TODO
        let default_conf = include_bytes!("../chat.toml");
        file.write_all(default_conf).unwrap(); // TODO
    }

    let config: Box<PluginConfig> = parse_toml_config(file).unwrap();
    config
}

fn file_exists(file_name: &str) -> bool {
    if let Ok(meta) = fs::metadata(file_name) {
        meta.is_file()
    } else {
        false
    }
}