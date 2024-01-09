use std::error::Error;
use crate::PLUGINS_FOLDER;
use serde::Deserialize;
use hutopia_utils::config::parse_toml_config;

pub fn parse_plugin_toml_config<ConfigType: for<'a> Deserialize<'a>>(
    plugin_name: &str,
) -> Result<Box<ConfigType>, Box<dyn Error>> {
    parse_toml_config(format!("{}/{}.toml", PLUGINS_FOLDER, plugin_name))
}