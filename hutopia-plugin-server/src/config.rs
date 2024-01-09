use crate::PLUGINS_FOLDER;
use hutopia_utils::config::parse_toml_config;
use serde::Deserialize;
use std::error::Error;

pub fn parse_plugin_toml_config<ConfigType: for<'a> Deserialize<'a>>(
    plugin_name: &str,
) -> Result<Box<ConfigType>, Box<dyn Error>> {
    parse_toml_config(format!("{}/{}.toml", PLUGINS_FOLDER, plugin_name))
}
