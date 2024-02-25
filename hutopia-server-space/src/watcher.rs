use actix_web::dev::ServerHandle;
use futures::executor::block_on;
use notify::Event;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::time::SystemTime;

/// Module to contain watcher related functions
/// Whenever a file in the plugins/ folder is modified, the server is restarted

#[derive(Default)]
pub struct WatcherData {
    pub handle: Option<ServerHandle>,
    pub needs_restart: bool,
}

#[macro_export]
macro_rules! start_server_with_plugins_watcher {
    ($server_data:expr, $bind_address:expr) => {
        let watcher_data: Arc<Mutex<WatcherData>> = Default::default();
        let (tx, rx) = channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
        watcher
            .watch(&Path::new("plugins"), RecursiveMode::Recursive)
            .unwrap();
        let inner_watcher_data = watcher_data.clone();
        thread::spawn(move || watcher::watcher_loop(inner_watcher_data, rx));

        loop {
            let needs_restart = {
                let mut data = watcher_data.lock().unwrap();
                let temp = (*data).needs_restart;
                (*data).needs_restart = false;
                // Returns the needs_restart value OR if the handle is None, that is
                // when the server has not started for the first time.
                temp || (*data).handle.is_none()
            };

            if needs_restart {
                start_server(
                    $server_data.clone(),
                    $bind_address.clone(),
                    watcher_data.clone(),
                )
                .await;
            } else {
                // Terminate the program completely
                std::process::exit(0);
            }
        }
    };
}

pub fn watcher_loop(
    inner_watcher_data: Arc<Mutex<WatcherData>>,
    rx: Receiver<Result<Event, notify::Error>>,
) {
    loop {
        let mut last_current_time = SystemTime::now();

        match rx.recv() {
            Ok(e) => {
                // Don't restart the server multiple times
                // if a file change triggers multiple events
                let current_time = SystemTime::now();
                if current_time - Duration::from_secs(1) >= last_current_time {
                    log::info!("Plugin file change detected, restarting server");

                    {
                        // set flag to true
                        let mut data = inner_watcher_data.lock().unwrap();
                        (*data).needs_restart = true;
                    }

                    // Restart server
                    let data = inner_watcher_data.lock().unwrap();
                    if let Some(data) = data.handle.as_ref().cloned() {
                        // Stop server gracefully
                        block_on(data.stop(false));
                    }
                }

                last_current_time = current_time;
            }
            Err(e) => std::process::exit(0),
        }
    }
}
