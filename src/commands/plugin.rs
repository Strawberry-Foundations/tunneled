#![allow(improper_ctypes_definitions)]

use libloading::{Library, Symbol};
use stblib::colors::{C_RESET, RED};
use stblib::external::plugin::{Plugin, PluginProperties};
use thiserror::Error;

#[repr(C)]
pub struct PluginCreateResult {
    pub plugin: *mut std::ffi::c_void,
    pub properties: PluginProperties,
}

type PluginCreate = unsafe extern "C" fn() -> PluginCreateResult;

pub struct LoadedPlugin {
    pub plugin: Box<dyn Plugin>,
    pub properties: PluginProperties,
    _lib: Library,
}

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("Failed to load library\n   -> \x1b[90m{0}\x1b[0m")]
    LoadError(String),
    #[error("Failed to load symbol\n   -> \x1b[90m{0}\x1b[0m")]
    SymbolError(String),
}

pub fn load_plugin(path: &str) -> Result<LoadedPlugin, PluginError> {
    let lib = unsafe { Library::new(path).map_err(|e| PluginError::LoadError(e.to_string()))? };
    let func: Symbol<PluginCreate> = unsafe {
        lib.get(b"create_plugin").map_err(|e| PluginError::SymbolError(e.to_string()))?
    };
    let result = unsafe { func() };
    let plugin: Box<dyn Plugin> = *unsafe { Box::from_raw(result.plugin as *mut Box<dyn Plugin>) };

    Ok(LoadedPlugin {
        plugin,
        properties: result.properties,
        _lib: lib,
    })
}

pub fn plugin() {
    let loaded = match load_plugin("plugins/example_plugin/target/debug/libexample_plugin.so") {
        Ok(plugin) => plugin,
        Err(err) => {
            eprintln!(" {RED}!{C_RESET} Plugin Error: {}", err);
            return;
        }
    };
    loaded.plugin.execute(&["test".to_string()]);
}
