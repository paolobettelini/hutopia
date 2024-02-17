use actix_web::middleware::DefaultHeaders;
use hutopia_database_relay::db::*;

use actix_files::Files;
use actix_web::*;
use hutopia_utils::config::parse_toml_config;

mod init;
mod config;
use init::*;
use config::*;

pub const LOG_ENV: &str = "RUST_LOG";

pub(crate) struct ServerData {
    db: Database,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    init_files();

    let config: Box<RelayConfig> = parse_toml_config("relay.toml").unwrap();
    let bind_address = (config.server.address, config.server.port);

    // init db
    let db_env = config.server.db_connection_env.clone();
    let url = match std::env::var(db_env) {
        Ok(v) => v,
        Err(e) => panic!("DB env variable not found")
    };
    let db = Database::new(url);

    HttpServer::new(move || {
        App::new()
            .wrap(
                DefaultHeaders::new()
                    // Set CORS headers
                    .add(("Access-Control-Allow-Origin", "*"))
                    .add((
                        "Access-Control-Allow-Methods",
                        "GET, POST, PUT, DELETE, OPTIONS",
                    ))
                    .add(("Access-Control-Allow-Headers", "Content-Type")),
            )
            .service(static_files)
            .app_data(web::Data::new(db.clone()))
    })
    .bind(&bind_address)?
    .run()
    .await
}

// Rust embed - TODO move to another file

use rust_embed::RustEmbed;
use mime_guess::from_path;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/../hutopia-frontend/dist"]
pub(crate) struct Asset;

#[get("/{_:.*}")]
async fn static_files(path: web::Path<String>) -> impl Responder {
    handle_static_file(path.as_str())
}

pub(crate) fn handle_static_file(path: &str) -> HttpResponse {
    // If in release mode, read from the embedded folder
    match Asset::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}