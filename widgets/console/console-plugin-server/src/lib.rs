use actix::Actor;
use actix_rt::{Arbiter, System};
use actix_web::web;
use hutopia_plugin_server::*;
use hutopia_plugin_server::utils::*;
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

struct AdminUser {
    username: String,
}

impl IPlugin for ConsolePlugin {

    fn init(&self, cfg: &mut ServiceConfig) {
        let path = format!("/widget/{}/cmd", PLUGIN_ID);
        let route = web::post().to(send_console_command);

        let config = config::get_config();
        let admin_user = AdminUser { username: config.plugin.admin_user.clone() };

        cfg.route(&path, route)
            .service(serve_widget_file)
            .app_data(web::Data::new(admin_user));
    }

    fn get_plugin_dependencies(&self) -> Vec<String> {
        vec![]
    }
}

async fn send_console_command(
    //path: web::Path<(String, String)>,
    req: HttpRequest,
) -> impl Responder {
    if let None = auth_user(&req) {
        return HttpResponse::Unauthorized().finish();
    }

    // TODO execute command
    
    HttpResponse::Ok().finish()
}

#[get("/widget/console/file/index.js")]
async fn serve_widget_file(
    admin_user: web::Data<AdminUser>,
    req: HttpRequest,
) -> impl Responder {
    // for some reason the REGEX gives a memory segfault
    //let filename = filename.to_string();
    let filename = String::from("index.js");
    let content = handle_static_file(&filename);

    if let Some(username) = auth_user(&req) {
        HttpResponse::Ok()
            .content_type(mime_guess::from_path(&filename).first_or_octet_stream().as_ref())
            .body(content)
    } else {
        // Don't serve console file
        HttpResponse::Ok().body(())
    }
}
