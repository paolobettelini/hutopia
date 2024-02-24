use actix::Actor;
use actix_rt::{Arbiter, System};
use actix_web::web;
use hutopia_plugin_server::*;
use rust_embed::RustEmbed;
use actix_web::web::ServiceConfig;
use actix_web::{HttpRequest, HttpResponse};
use actix_web::Responder;

mod config;
use config::*;

const PLUGIN_ID: &str = "console";

hutopia_plugin_server::export_plugin!(register);

extern "C" fn register(registrar: &mut dyn IPluginRegistrar) {
    let pl = ConsolePlugin;
    registrar.register_plugin(PLUGIN_ID, Box::new(pl));
}

#[derive(RustEmbed)]
#[folder = "../console-frontend/"]
pub(crate) struct DistAsset;

pub(crate) fn handle_static_file(path: &str) -> Vec<u8> {
    match DistAsset::get(path) {
        Some(content) => content.data.into_owned(),
        None => vec![], // TODO return Option
    }
}

#[derive(Debug)]
pub struct ConsolePlugin;

impl IPlugin for ConsolePlugin {
    fn get_file(&self, file_name: &str) -> Vec<u8> {
        handle_static_file(&file_name)
    }

    fn init(&self, cfg: &mut ServiceConfig) {
        let path = format!("/widget/{}/ws", PLUGIN_ID);
        let route = web::post().to(send_console_command);

        cfg.route(&path, route);
    }

    fn get_plugin_dependencies(&self) -> Vec<String> {
        vec![]
    }
}

async fn send_console_command(
    //data: web::Data<ServerData>,
    //path: web::Path<(String, String)>,
    req: HttpRequest,
) -> impl Responder {
    HttpResponse::Ok()
}