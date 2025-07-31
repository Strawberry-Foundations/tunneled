//! A modern, simple TCP tunnel in Rust that exposes local ports to a remote
//! server, bypassing standard NAT connection firewalls.
//!
//! This is the library crate documentation. If you're looking for usage
//! information about the binary, see the command below.
//!
//! ```shell
//! $ tunneled help
//! ```
//!
//! There are two components to the crate, offering implementations of the
//! server network daemon and client local forwarding proxy. Both are public
//! members and can be run programmatically with a Tokio 1.0 runtime.
use anyhow::Result;
use libstrawberry::colors::{BOLD, C_RESET, RED, RESET};

use crate::cli::args::Command;
use crate::cli::{ARGS, OPTIONS};
use crate::commands::auth::auth;
use crate::commands::compose::compose;
use crate::commands::local::Client;
use crate::commands::server::{read_config_file, Server};
use crate::core::auth::Auth;

pub mod cli;
pub mod commands;
pub mod core;

#[tokio::main]
async fn main() -> Result<()> {
    match ARGS.command {
        Command::Local => {
            let client = Client::new(
                &OPTIONS.client_options.host,
                OPTIONS.client_options.port,
                &OPTIONS.client_options.server,
                OPTIONS.client_options.secret.as_deref(),
                OPTIONS.client_options.static_port,
                OPTIONS.client_options.control_port,
                OPTIONS.client_options.auth,
                None,
            )
            .await
            .unwrap_or_else(|err| {
                eprintln!("{RED}{BOLD} ! {C_RESET} {err}");
                std::process::exit(1)
            });

            client.listen().await.unwrap_or_else(|err| {
                eprintln!("{RED}{BOLD} ! {C_RESET} {err}");
                std::process::exit(1)
            });
        }
        Command::Compose => compose(OPTIONS.client_options.compose_file.as_deref())
            .await
            .unwrap_or_else(|err| {
                eprintln!("{RED}{BOLD} ! {C_RESET} {err}");
                std::process::exit(1)
            }),
        Command::Server => {
            if let Some(config_file) = OPTIONS.server_options.config_file.as_deref() {
                let config = read_config_file(config_file).unwrap_or_else(|err| {
                    eprintln!("{RED}{BOLD} ! {C_RESET} {err}");
                    std::process::exit(1)
                });

                let port_range = config.server.host.min_port..=config.server.host.max_port;
                if port_range.is_empty() {
                    eprintln!("{RED}{BOLD} ! {RESET} Port range is empty{C_RESET}");
                }
                Server::new(
                    port_range,
                    config.server.auth.secret.as_deref(),
                    config.server.host.control_port.unwrap_or(7835),
                    config.server.auth.require_id.unwrap_or(false),
                    config.server.auth.allow_static_port.unwrap_or_default(),
                    config
                        .server
                        .host
                        .tunnels_addr
                        .as_deref()
                        .unwrap_or("0.0.0.0")
                        .to_string(),
                )
                .listen()
                .await?;
            } else {
                let port_range = OPTIONS.server_options.min_port..=OPTIONS.server_options.max_port;
                if port_range.is_empty() {
                    eprintln!("{RED}{BOLD} ! {RESET} Port range is empty{C_RESET}");
                }

                Server::new(
                    port_range,
                    OPTIONS.server_options.secret.as_deref(),
                    OPTIONS.server_options.control_port,
                    OPTIONS.server_options.require_id,
                    Vec::new(),
                    OPTIONS.server_options.tunnels_addr.clone(),
                )
                .listen()
                .await?;
            }
        }
        Command::Auth => auth(Auth::strawberry_id()).await?,
        Command::About => commands::about::about(),
        Command::Plugin => commands::plugin::plugin()?,
        Command::None => commands::help::help(),
    }

    Ok(())
}
