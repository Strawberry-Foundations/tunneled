#![allow(improper_ctypes_definitions)]

use std::{env, fs};
use libloading::{Library, Symbol};
use libstrawberry::colors::{C_RESET, RED, BOLD, RESET, CYAN, GREEN, UNDERLINE, WHITE, MAGENTA};
use libstrawberry::external::plugin::{Plugin, PluginProperties};
use thiserror::Error;
use crate::core::constants::VERSION;

type PluginCreate = unsafe extern "C" fn() -> (Box<dyn Plugin>, PluginProperties);

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


pub fn help() {
    println!("\
{BOLD}{CYAN}{UNDERLINE}Strawberry Tunneled v{} [PLUGIN]{C_RESET}\n\
{GREEN}{BOLD}Usage:{RESET} {WHITE}tunneled plugin {CYAN}[command] {RED}[<options>]{C_RESET}\n\n\
{MAGENTA}{BOLD}Commands:{C_RESET}
    {CYAN}{BOLD}list:{C_RESET} Lists all available plugins
    {CYAN}{BOLD}help:{C_RESET} Prints this message

    {CYAN}{BOLD}<plugin-id> [<options>]:{C_RESET} Run a plugin by its ID
", *VERSION);
    std::process::exit(0);
}

pub fn load_plugin(path: &str) -> Result<LoadedPlugin, PluginError> {
    let lib = unsafe {
        Library::new(path).map_err(|e| PluginError::LoadError(e.to_string()))?
    };
    let func: Symbol<PluginCreate> = unsafe {
        lib.get(b"create_plugin")
            .map_err(|e| PluginError::SymbolError(e.to_string()))?
    };
    let (plugin, properties) = unsafe { func() };

    Ok(LoadedPlugin {
        plugin,
        properties,
        _lib: lib,
    })
}

pub fn get_plugins() -> anyhow::Result<Box<Vec<LoadedPlugin>>> {
    let mut plugins = Vec::new();
    
    let plugin_directory = if let Some(home_dir) = dirs::home_dir() {
        let plugin_dir = home_dir.join(".config").join("tunneled").join("plugins");

        if plugin_dir.exists() {
            plugin_dir
        } else {
            fs::create_dir_all(&plugin_dir).expect("Failed to create plugin directory");
            return Err(anyhow::anyhow!(
                "{}{}Error while fetching plugin directory:{} Plugin directory does not exist. Please run `tunneled plugin install` to install plugins.{}",
                RED, BOLD, RESET, C_RESET
            ));
        }
    } else {
        anyhow::bail!(
            "{}{}Error while fetching plugin directory:{} Home directory not found.{}",
            RED, BOLD, RESET, C_RESET
        );
    };

    match fs::read_dir(plugin_directory) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if !path.is_dir() {
                            let loaded = match load_plugin(&path.to_string_lossy()) {
                                Ok(plugin) => plugin,
                                Err(err) => {
                                    eprintln!(" {RED}!{C_RESET} Plugin Error: {err}");
                                    return Err(anyhow::anyhow!(""))
                                }
                            };
                            
                            plugins.push(loaded);
                        }
                        
                    }
                    Err(e) => println!("{e}"),
                }
            }
        }
        Err(e) => println!("{e}"),
    }

    Ok(Box::new(plugins))
}


pub fn list() -> anyhow::Result<()> {
    let plugins = get_plugins()?;
    
    if plugins.is_empty() {
        println!("{RED}{BOLD}No plugins found. Please run `tunneled plugin install` to install plugins.{C_RESET}");
        return Ok(());
    }
    
    for plugin in plugins.iter() {
        println!("{BOLD}* {CYAN}{} ({}){C_RESET}", plugin.properties.name, plugin.properties.id);
        println!("   - Name: {GREEN}{BOLD}{}{C_RESET}", plugin.properties.name);
        println!("   - Version: {GREEN}{BOLD}{}{C_RESET}", plugin.properties.version);
        println!("   - Package ID: {GREEN}{BOLD}{}{C_RESET}", plugin.properties.package_id);
        println!("   - Library version: {GREEN}{BOLD}{}{C_RESET}", plugin.properties.library_version);
    }
    
    Ok(())
}

pub fn plugin() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().skip(2).collect();
    let plugins = get_plugins()?;

    if args.is_empty() {
        help();
        return Ok(());
    }
    
    match args.first().unwrap_or_else(|| std::process::exit(1)).as_str() {
        "list" => {
            if let Err(e) = list() {
                eprintln!("{RED}{BOLD}Error while listing plugins:{RESET} {e}{C_RESET}");
            }
            Ok(())
        },
        &_ => {
            let plugin_id = args.first().expect("No plugin ID provided");
            if let Some(loaded) = plugins.iter().find(|p| p.properties.id == *plugin_id) {
                loaded.plugin.execute(&args[1..]);
            } else {
                eprintln!("{RED}{BOLD}Plugin with id '{plugin_id}' not found.{C_RESET}");
            }
            Ok(())
        }
    }
}
