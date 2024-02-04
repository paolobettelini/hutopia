use crate::*;
use std::fs;
use std::io::Write;

pub fn init_logger() {
    if std::env::var(LOG_ENV).is_err() {
        std::env::set_var(LOG_ENV, "info");
    }

    env_logger::init();
}

pub fn init_files() {

    // Create config file if it doesn't exist
    if !file_exists("relay.toml") {
        log::info!("Created config file: {}", "relay.toml");
        let mut file = fs::File::create("relay.toml").unwrap(); // TODO
        file.write_all(include_bytes!("../relay.toml")).unwrap(); // TODO
    }
}

fn folder_exists(folder_name: &str) -> bool {
    if let Ok(meta) = fs::metadata(folder_name) {
        meta.is_dir()
    } else {
        false
    }
}

fn file_exists(file_name: &str) -> bool {
    if let Ok(meta) = fs::metadata(file_name) {
        meta.is_file()
    } else {
        false
    }
}
