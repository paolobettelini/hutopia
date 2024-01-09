use std::fs;
use std::io::Write;
use crate::*;

pub fn init_logger() {
    if std::env::var(LOG_ENV).is_err() {
        std::env::set_var(LOG_ENV, "info");
    }

    env_logger::init();
}

pub fn init_files() {
    // Create plugins folder if it doesn't exist
    if !folder_exists(PLUGINS_FOLDER) {
        match fs::create_dir(PLUGINS_FOLDER) {
            Ok(_) => log::info!("Created plugins folder: {}", PLUGINS_FOLDER),
            Err(e) => log::warn!("Failed to create plugins folder: {}", e),
        }
    }

    // Create website folder if it doesn't exist
    if !folder_exists("space") {
        match fs::create_dir("space") {
            Ok(_) => {
                log::info!("Created space folder: {}", "space");

                // Add default file index.html
                let mut file = fs::File::create("space/index.html").unwrap(); // TODO
                file.write_all(include_bytes!("../space/index.html")).unwrap(); // TODO
            },
            Err(e) => log::warn!("Failed to create space folder: {}", e),
        }
    }

    // Create config file if it doesn't exist
    if !file_exists("hutopia.toml") {
        let mut file = fs::File::create("hutopia.toml").unwrap(); // TODO
        file.write_all(include_bytes!("../hutopia.toml")).unwrap(); // TODO
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