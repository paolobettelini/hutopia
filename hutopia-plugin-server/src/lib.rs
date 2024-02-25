use actix_web::web::ServiceConfig;

mod plugins;
pub mod utils;
pub use plugins::*;
pub mod config;

pub const PLUGINS_FOLDER: & str = "./plugins";

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub trait IPlugin {
    fn init(&self, cfg: &mut ServiceConfig);

    fn get_plugin_dependencies(&self) -> Vec<String>;
}

pub trait IPluginRegistrar {
    fn register_plugin(&mut self, name: &str, implementation: Box<dyn IPlugin>);
}

#[derive(Copy, Clone)]
pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn IPluginRegistrar),
}

#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::PluginDeclaration = $crate::PluginDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            core_version: $crate::CORE_VERSION,
            register: $register,
        };
    };
}
