use actix::{Actor, StreamHandler};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use hutopia_plugin_core::IPlugin;
use mime_guess::from_path;
use std::alloc::System;

mod plugins;
use plugins::*;

const ADDRESS: (&'static str, u16) = ("0.0.0.0", 8080);
const PLUGINS_FOLDER: &'static str = "./plugins";
const LOG_ENV: &str = "RUST_LOG";

#[global_allocator]
static ALLOCATOR: System = System;

pub struct ServerData {
    pub plugin_handler: PluginHandler,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if std::env::var(LOG_ENV).is_err() {
        std::env::set_var(LOG_ENV, "info");
    }

    env_logger::init();

    HttpServer::new(move || {
        let data = web::Data::new(get_data()); // Internally an Arc

        let mut app = App::new()
            .wrap( // Set CORS headers
                actix_web::middleware::DefaultHeaders::new()
                    .add(("Access-Control-Allow-Origin", "*"))
                    .add(("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS"))
                    .add(("Access-Control-Allow-Headers", "Content-Type"))
            )
            .service(serve_widget_file)
            .app_data(data.clone());
        
        // Configure plugins
        for plugin in data.plugin_handler.plugins.values() {
            app = app.configure(|cfg| plugin.config(cfg));
        }

        app
    })
    // IMPORTANT!
    // The HttpServer normally has as many workers as available (https://docs.rs/actix-web/latest/actix_web/struct.HttpServer.html#worker-count)
    // Thus, every plugin is initialized {cores} times, separating the instances (if the plugin uses
    // an Arbiter, there will be multiple arbiters and the actors are also separated).
    // So, I use just 1 thread for the HttpServer.
    // `libloading::Library` cannot be shared between threads.
    .workers(1)
    .bind(ADDRESS)?
    .run()
    .await?;

    Ok(())
}

fn get_data() -> ServerData {
    let mut plugin_handler = PluginHandler::new();

    if let Ok(entries) = std::fs::read_dir(PLUGINS_FOLDER) {
        for entry in entries.flatten() {
            if let Ok(file_path) = entry.path().into_os_string().into_string() {
                if file_path.ends_with(".so") {
                    unsafe {
                        plugin_handler
                        .load(file_path)
                        .expect("Plugin loading failed");
                    }
                }
            }
        }
    }

    ServerData { plugin_handler }
}

#[get("/widget_file/{widget_name}/{file_name:.+}")]
async fn serve_widget_file(
    data: web::Data<ServerData>,
    params: web::Path<(String, String)>,
) -> impl Responder {
    let widget_name = format!("{}", params.0);
    let file_name = format!("{}", params.1);

    // TODO: maybe directly register /widget_file/example/{file} at boot

    let content = data
        .plugin_handler
        .plugins
        .get(&widget_name)
        .unwrap()
        .get_file(&file_name);

    HttpResponse::Ok()
        .content_type(from_path(&file_name).first_or_octet_stream().as_ref())
        .body(content)
}