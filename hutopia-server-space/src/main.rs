use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware::DefaultHeaders};
use actix_web::cookie::{Key, SameSite};
use hutopia_plugin_server::*;
use hutopia_utils::config::*;
use mime_guess::from_path;
use std::alloc::System;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use actix_session::{
    config::CookieContentSecurity, storage::CookieSessionStore, SessionMiddleware,
};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    init_files();

    // init config
    let config: Box<SpaceConfig> = parse_toml_config("space.toml").unwrap();
    let bind_address = (config.server.address.clone(), config.server.port);

    HttpServer::new(move || {
        // Server data (internally an Arc)
        // Note, this cannot be created outside since PluginHandler
        // is not thread-safe.
        let server_data = web::Data::new(ServerData::new(&config));

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
            .service(serve_widget_file)
            .service(serve_space_file)
            .app_data(server_data.clone());

        // Init plugins
        for plugin in server_data.plugin_handler.plugins.values() {
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
