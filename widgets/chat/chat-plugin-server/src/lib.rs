use actix::Actor;
use actix_rt::{Arbiter, System};
use actix_web::web;
use hutopia_plugin_server::*;
use rust_embed::RustEmbed;
use chat_plugin_database::db::Database;
use actix_web::web::ServiceConfig;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::get;

mod actors_messages;
mod chat;
mod websocket;
mod config;
pub(crate) use actors_messages::*;
use chat::*;
use websocket::*;

const PLUGIN_ID: &str = "chat";

hutopia_plugin_server::export_plugin!(register);

extern "C" fn register(registrar: &mut dyn IPluginRegistrar) {
    // New system for Arbiter
    let _ = System::new();
    let arbiter = Arbiter::new();

    let pl = ChatPlugin { arbiter };
    registrar.register_plugin(PLUGIN_ID, Box::new(pl));
}

#[derive(RustEmbed)]
#[folder = "../dist/"]
pub(crate) struct DistAsset;

pub(crate) fn handle_static_file(path: &str) -> Vec<u8> {
    match DistAsset::get(path) {
        Some(content) => content.data.into_owned(),
        None => vec![], // TODO return Option
    }
}

#[derive(Debug)]
pub struct ChatPlugin {
    arbiter: Arbiter,
}

impl IPlugin for ChatPlugin {
    fn init(&self, cfg: &mut ServiceConfig) {
        // Init sessions handler actor
        let addr = Chat::start_in_arbiter(&self.arbiter.handle(), |_| {
            Chat::new()
        });

        // Plugin Websocket route
        let path = format!("/widget/{}/ws", PLUGIN_ID);
        let route = web::get().to(init_connection);

        cfg.route(&path, route)
            .service(serve_widget_file)
            .app_data(web::Data::new(addr.clone()));
    }

    fn get_plugin_dependencies(&self) -> Vec<String> {
        vec![]
    }
}

#[get("/widget/chat/file/{filename:.+}")]
async fn serve_widget_file(
    filename: web::Path<String>,
) -> impl Responder {
    let filename = filename.to_string();
    let content = handle_static_file(&filename);

    HttpResponse::Ok()
        .content_type(mime_guess::from_path(&filename).first_or_octet_stream().as_ref())
        .body(content)
}