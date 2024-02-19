use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use hutopia_plugin_server::*;
use hutopia_utils::config::*;
use mime_guess::from_path;
use std::alloc::System;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod config;
mod init;
mod lib_ext;
use config::*;
use init::*;
use lib_ext::*;

pub const LOG_ENV: &str = "RUST_LOG";

#[global_allocator]
static ALLOCATOR: System = System;

pub struct ServerData {
    pub plugin_handler: PluginHandler,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    init_files();

    // init config
    let config: Box<SpaceConfig> = parse_toml_config("space.toml").unwrap();
    let bind_address = (config.server.address, config.server.port);

    // Server data
    //let server_data = ServerData::new(&config);

    HttpServer::new(move || {
        let data = web::Data::new(get_data()); // Internally an Arc

        let mut app = App::new()
            .wrap(
                // Set CORS headers
                actix_web::middleware::DefaultHeaders::new()
                    .add(("Access-Control-Allow-Origin", "*"))
                    .add((
                        "Access-Control-Allow-Methods",
                        "GET, POST, PUT, DELETE, OPTIONS",
                    ))
                    .add(("Access-Control-Allow-Headers", "Content-Type")),
            )
            .service(serve_widget_file)
            .service(serve_space_file)
            .app_data(data.clone());

        // Init plugins
        for plugin in data.plugin_handler.plugins.values() {
            app = app.configure(|cfg| plugin.init(cfg));
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
    .bind(bind_address)?
    .run()
    .await?;

    Ok(())
}

fn get_data() -> ServerData {
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

    ServerData { plugin_handler }
}

#[get("/widget/{widget_name}/file/{file_name:.+}")]
async fn serve_widget_file(
    data: web::Data<ServerData>,
    params: web::Path<(String, String)>,
) -> impl Responder {
    let widget_name = params.0.to_string();
    let file_name = params.1.to_string();

    // TODO: maybe directly register /widget/chat/file/{file} at boot

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

#[get("/space_file/{file_name:.+}")]
async fn serve_space_file(
    _data: web::Data<ServerData>,
    params: web::Path<String>,
) -> impl Responder {
    let file_name = params.to_string();
    let full_path = Path::new("space").join(&file_name);
    let mut file = File::open(&full_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    HttpResponse::Ok()
        .content_type(from_path(&file_name).first_or_octet_stream().as_ref())
        .body(content)
}
