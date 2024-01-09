use serde::Deserialize;
use std::{fs, path::Path};
use std::error::Error;
use crate::PLUGINS_FOLDER;

pub fn parse_plugin_toml_config<ConfigType: for<'a> Deserialize<'a>>(
    plugin_name: &str,
) -> Result<Box<ConfigType>, Box<dyn Error>> {
    parse_toml_config(format!("{}/{}.toml", PLUGINS_FOLDER, plugin_name))
}


fn parse_toml_config<P: AsRef<Path>, ConfigType: for<'a> Deserialize<'a>>(
    config_path: P,
) -> Result<Box<ConfigType>, Box<dyn Error>> {
    let content = fs::read_to_string(config_path)?;

    let config = toml::from_str(&content)?;

    Ok(Box::new(config))
}
