use actix_web::middleware::DefaultHeaders;
use hutopia_database_relay::db::*;

use actix_files::Files;
use actix_session::{
    config::CookieContentSecurity, storage::CookieSessionStore, Session, SessionMiddleware,
};
use actix_web::cookie::{Key, SameSite};
use actix_web::*;

use hutopia_utils::config::parse_toml_config;

mod auth;
mod config;
mod init;
mod routes;
mod state;
use config::*;
use init::*;
use routes::*;
use state::*;

pub const LOG_ENV: &'static str = "RUST_LOG";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    init_files();

    // init config
    let config: Box<RelayConfig> = parse_toml_config("relay.toml").unwrap();
    let bind_address = (config.server.address.clone(), config.server.port);

    // Server data
    let server_data = ServerData::new(&config);

    HttpServer::new(move || {
        App::new()
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
            .service(login)
            .service(register)
            .service(login_fallback)
            .service(user_data)
            .service(static_files)
            .app_data(web::Data::new(server_data.clone()))
    })
    .bind(&bind_address)?
    .run()
    .await
}

use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/../hutopia-frontend/dist"]
pub(crate) struct Asset;

#[get("/{_:.*}")]
async fn static_files(path: web::Path<String>) -> impl Responder {
    handle_static_file(path.as_str())
}

pub(crate) fn handle_static_file(path: &str) -> HttpResponse {
    let mut path = path;
    if path == "" {
        path = "index.html";
    }

    // read from the embedded folder
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}
