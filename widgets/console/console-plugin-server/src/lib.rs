use actix::Actor;
use actix_rt::{Arbiter, System};
use actix_web::web;
use hutopia_plugin_server::*;
use rust_embed::RustEmbed;
use actix_web::web::ServiceConfig;
use actix_web::{HttpRequest, HttpResponse};
use actix_web::Responder;
use actix_web::get;

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

    fn init(&self, cfg: &mut ServiceConfig) {
        let path = format!("/widget/{}/ws", PLUGIN_ID);
        let route = web::post().to(send_console_command);

        cfg.route(&path, route)
            .service(serve_widget_file);
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

#[get("/widget/console/file/{filename:.+}")]
async fn serve_widget_file(
    filename: web::Path<String>,
) -> impl Responder {
    let filename = filename.to_string();
    let content = handle_static_file(&filename);

    HttpResponse::Ok()
        .content_type(mime_guess::from_path(&filename).first_or_octet_stream().as_ref())
        .body(content)
}