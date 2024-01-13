use actix::Actor;
use actix_rt::{Arbiter, System};
use actix_web::web;
use hutopia_plugin_server::*;
use rust_embed::RustEmbed;
use chat_plugin_database::db::Database;

use actix_web::web::ServiceConfig;

mod actors_messages;
mod chat;
mod websocket;
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
#[folder = "../pkg/"]
pub(crate) struct PkgAsset;

pub(crate) fn handle_static_file(path: &str) -> Vec<u8> {
    match PkgAsset::get(path) {
        Some(content) => content.data.into_owned(),
        None => vec![], // TODO return Option
    }
}

#[derive(Debug)]
pub struct ChatPlugin {
    arbiter: Arbiter,
}

impl IPlugin for ChatPlugin {
    fn get_file(&self, file_name: &str) -> Vec<u8> {
        handle_static_file(&file_name)
    }

    fn init(&self, cfg: &mut ServiceConfig) {
        // Init sessions handler actor
        let addr = Chat::start_in_arbiter(&self.arbiter.handle(), |_| {
            // init db
            // TODO read env
            let url = String::from("XXX");
            let db = Database::new(url);
            Chat::new(db, Default::default())
        });

        // Plugin Websocket route
        let path = format!("/widget_ws/{}", PLUGIN_ID);
        let route = web::get().to(init_connection);

        cfg.route(&path, route)
            .app_data(web::Data::new(addr.clone()));
    }
}
