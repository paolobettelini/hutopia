use actix_session::{
    config::CookieContentSecurity, storage::CookieSessionStore, SessionMiddleware,
};
use actix_web::cookie::{Key, SameSite};
use actix_web::{
    get, guard, middleware::DefaultHeaders, post, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use hutopia_plugin_server::*;
use hutopia_utils::config::*;
use mime_guess::from_path;
use std::alloc::System;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde_json::json;

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
            .service(internal_user_auth)
            .app_data(server_data.clone());

        // Init plugins
        for plugin in server_data.plugin_handler.plugins.values() {
            app = app.configure(|cfg| plugin.init(cfg));
        }

        app
    })
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

    // TODO check cache, otherwise do query
    // HashMap<User, List<Token>>

    // When we do this query, the token is consumated and removed from the relay,
    // so it's important to cache this information for a minute, so that each plugin
    // can authenticate the user. This also mean that plugin should authenticate the
    // user immediately and you cannot dynamically load widgets in the space page.
    let authenticated = data.auth_user(username, token).await;
    log::info!("User is authenticated: {authenticated}");

    // TODO Add to cache
    // Todo: use 1 minute TimedCache

    let json = json!({
        "authenticated": authenticated
    });

    HttpResponse::Ok().json(json)
}
