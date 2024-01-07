use hutopia_plugin_core::*;
use rust_embed::RustEmbed;
use actix_web::{Route, web};
use actix::{Actor};
use actix_rt::{Arbiter, System};
use std::sync::Arc;
use actix_web::web::ServiceConfig;

mod websocket;
mod chat;
mod actors_messages;
use websocket::*;
use chat::*;
pub(crate) use actors_messages::*;

const PLUGIN_ID: &str = "example";

hutopia_plugin_core::export_plugin!(register);

extern "C" fn register(registrar: &mut dyn IPluginRegistrar) {
    // New system for Arbiter
    let _  = System::new(); 
    let arbiter = Arbiter::new();

    let pl = PluginExample { arbiter };
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
pub struct PluginExample {
    arbiter: Arbiter,
}

impl IPlugin for PluginExample {
    fn get_file(&self, file_name: &str) -> Vec<u8> {
        handle_static_file(&file_name)
    }

    fn config(&self, cfg: &mut ServiceConfig) {
        // Init sessions handler actor
        let addr = Chat::start_in_arbiter(&self.arbiter.handle(), |_| Chat::default());

        // Plugin Websocket route
        let path = format!("/widget_ws/{}", PLUGIN_ID);
        let route = web::get().to(init_connection);

        cfg
            .route(&path, route)
            .app_data(web::Data::new(addr.clone()));
    }
}