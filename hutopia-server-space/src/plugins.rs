use hutopia_plugin_core::*;
use libloading::Library;
use std::{collections::HashMap, ffi::OsStr, io, rc::Rc};
use actix_web::Route;
use std::sync::Arc;
use actix_web::web::ServiceConfig;

/// A map of the plugins.
#[derive(Default)]
pub struct PluginHandler {
    pub plugins: HashMap<String, PluginProxy>,
    libraries: Vec<Rc<Library>>,
}

impl PluginHandler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load a plugin library and add all contained functions to the internal
    /// plugins table.
    ///
    /// # Safety
    ///
    /// A plugin library **must** be implemented using the
    /// [`plugin_core::plugin_declaration!()`] macro. Trying manually implement
    /// a plugin without going through that macro will result in undefined
    /// behaviour.
    pub unsafe fn load<P: AsRef<OsStr>>(&mut self, library_path: P) -> io::Result<()> {
        // load the library into memory
        let library = Rc::new(Library::new(library_path).unwrap());

        // get a pointer to the plugin_declaration symbol.
        let decl = library
            .get::<*mut PluginDeclaration>(b"plugin_declaration\0")
            .unwrap()
            .read();

        // version checks to prevent accidental ABI incompatibilities
        if decl.rustc_version != hutopia_plugin_core::RUSTC_VERSION
            || decl.core_version != hutopia_plugin_core::CORE_VERSION
        {
            return Err(io::Error::new(io::ErrorKind::Other, "Version mismatch"));
        }

        let mut registrar = PluginRegistrar::new(Rc::clone(&library));

        (decl.register)(&mut registrar);

        // add all loaded plugins to the functions map
        self.plugins.extend(registrar.plugins);
        // and make sure PluginHandler keeps a reference to the library
        self.libraries.push(library);

        Ok(())
    }
}

struct PluginRegistrar {
    plugins: HashMap<String, PluginProxy>,
    lib: Rc<Library>,
}

impl PluginRegistrar {
    fn new(lib: Rc<Library>) -> PluginRegistrar {
        PluginRegistrar {
            lib,
            plugins: HashMap::default(),
        }
    }
}

impl IPluginRegistrar for PluginRegistrar {
    fn register_plugin(&mut self, name: &str, plugin: Box<dyn IPlugin>) {
        log::info!("Loading plugin: {}", name);
        let proxy = PluginProxy {
            plugin,
            _lib: Rc::clone(&self.lib),
        };

        self.plugins.insert(name.to_string(), proxy);
    }
}

/// A proxy object which wraps a [`Plugin`] and makes sure it can't outlive
/// the library it came from.
pub struct PluginProxy {
    plugin: Box<dyn IPlugin>,
    _lib: Rc<Library>,
}

impl IPlugin for PluginProxy {
    fn get_file(&self, file_name: &str) -> Vec<u8> {
        self.plugin.get_file(file_name)
    }

    fn config(&self, cfg: &mut ServiceConfig) {
        self.plugin.config(cfg)
    }
}
