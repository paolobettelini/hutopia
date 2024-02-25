use actix_session::{
    config::CookieContentSecurity, storage::CookieSessionStore, SessionMiddleware,
};
use std::time::SystemTime;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use actix_web::cookie::{Key, SameSite};
use actix_web::{
    get, guard, middleware::DefaultHeaders, post, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use actix_web::dev::ServerHandle;
use hutopia_plugin_server::*;
use hutopia_utils::config::*;
use mime_guess::from_path;
use std::alloc::System;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde_json::json;
use std::thread;
use std::time::Duration;
use futures::executor::block_on;

mod config;
mod init;
mod lib_ext;
mod state;
use config::*;
use init::*;
use lib_ext::*;
use state::*;

pub const LOG_ENV: &str = "RUST_LOG";

#[global_allocator]
static ALLOCATOR: System = System;

#[derive(Default)]
struct WatcherData {
    handle: Option<ServerHandle>,
    needs_restart: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    init_files();

    // init config
    let config: Box<SpaceConfig> = parse_toml_config("space.toml").unwrap();
    let bind_address = (config.server.address.clone(), config.server.port);
    
    let server_data = web::Data::new(ServerData::new(&config));

    let watcher_data: Arc<Mutex<WatcherData>> = Arc::new(Mutex::new(Default::default()));
    
    // Watch for file changes in the "plugins/" folder
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
    let plugins_folder = Path::new("plugins");
    watcher.watch(&plugins_folder, RecursiveMode::Recursive).unwrap();

    let inner_watcher_data = watcher_data.clone();
    thread::spawn(move || {
        loop {
            let mut last_current_time = SystemTime::now();

            match rx.recv() {
                Ok(e) => {
                    // Don't restart the server multiple times
                    // if a file change triggers multiple events
                    let current_time = SystemTime::now();
                    if current_time - Duration::from_secs(1) >= last_current_time {
                        log::info!("Plugin file change detected, restarting server");
                        
                        { // set flag to true
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
    });

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
            start_server(server_data.clone(), bind_address.clone(), watcher_data.clone()).await;
        } else {
            std::process::exit(0);
        }
    }

    Ok(())
}

async fn start_server(
    server_data: web::Data<ServerData>,
    bind_address: (String, u16),
    watcher_data: Arc<Mutex<WatcherData>>,
) {
    let server = HttpServer::new(move || {
        // `PluginHandler` is initialized for each thread since it is not thread safe.
        // Plugins must share data statically
        let plugin_handler = web::Data::new(load_plugin_handler());

        let mut app = App::new()
            // Set CORS headers
            // this allows the client to make requests to
            // third-party spaces.
            .wrap(
                DefaultHeaders::new()
                    .add(("Access-Control-Allow-Origin", "*"))
                    .add((
                        "Access-Control-Allow-Methods",
                        "GET, POST, PUT, DELETE, OPTIONS",
                    ))
                    .add(("Access-Control-Allow-Headers", "Content-Type")),
            )
            // This prevents CSRF attacks
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::generate())
                    .cookie_content_security(CookieContentSecurity::Private)
                    .cookie_same_site(SameSite::Lax)
                    .build(),
            )
            .service(serve_space_file)
            .service(internal_user_auth)
            .app_data(server_data.clone())
            .app_data(plugin_handler.clone());

        // Init plugins
        for plugin in plugin_handler.plugins.values() {
            app = app.configure(|cfg| plugin.init(cfg));
        }

        app
    })
    .bind(bind_address)
    .unwrap()
    .run();

    { // Update handle reference
        let handle = server.handle();
        let mut data = watcher_data.lock().unwrap();
        (*data).handle = Some(handle);
    }

    server
        .await
        .unwrap();
}

#[get("/space_file/{file_name:.+}")]
async fn serve_space_file(params: web::Path<String>) -> impl Responder {
    let file_name = params.to_string();
    let full_path = Path::new("space").join(&file_name);
    let mut file = File::open(&full_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    HttpResponse::Ok()
        .content_type(from_path(&file_name).first_or_octet_stream().as_ref())
        .body(content)
}

#[post("/internal/auth/{username}/{token}")]
async fn internal_user_auth(
    data: web::Data<ServerData>,
    path: web::Path<(String, String)>,
    req: HttpRequest,
) -> impl Responder {
    // Only allow requests from loopback address
    if !req.peer_addr().map_or(false, |remote| {
        remote.ip().to_string() == "127.0.0.1"
    }) {
        log::warn!("Remote peer is not loopback");
        return HttpResponse::NotFound().finish();
    }

    let (username, token) = (&path.0, &path.1);

    let authenticated = data.auth_user(username, token).await;
    log::info!("User is authenticated: {authenticated}");

    let json = json!({
        "authenticated": authenticated
    });

    HttpResponse::Ok().json(json)
}

fn load_plugin_handler() -> PluginHandler {
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

    plugin_handler.ensure_dependencies();

    plugin_handler
}