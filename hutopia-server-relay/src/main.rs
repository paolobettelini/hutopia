use actix_web::middleware::DefaultHeaders;
use hutopia_database_relay::db::*;

use actix_files::Files;
use actix_web::*;
use hutopia_server_relay::app::*;
use leptos::*;
use leptos_actix::{generate_route_list, LeptosRoutes};
use hutopia_utils::config::parse_toml_config;

mod init;
mod config;
use init::*;
use config::*;

// TODO conf
const DB_CONNECTION_URL: &str = "postgresql://worker:pass@ip:5432/hutopia";
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

    let leptos_conf = get_configuration(None).await.unwrap();
   
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    HttpServer::new(move || {
        let leptos_options = &leptos_conf.leptos_options;
        let site_root = &leptos_options.site_root;

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
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.app_data(web::Data::new(get_server_data()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&bind_address)?
    .run()
    .await
}

fn get_server_data() -> ServerData {
    let db = Database::new(DB_CONNECTION_URL.to_string());

    ServerData { db }
}

#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}