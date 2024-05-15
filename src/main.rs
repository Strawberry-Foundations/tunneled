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
use stblib::colors::{BOLD, C_RESET, RED, RESET};
use crate::auth::Auth;

use crate::commands::client::Client;
use crate::commands::server::Server;
use crate::commands::auth::auth;

use crate::cli::{ARGS, OPTIONS};
use crate::cli::args::Command;

pub mod cli;
pub mod commands;
pub mod statics;
pub mod auth;
pub mod shared;

#[tokio::main]
async fn main() -> Result <()> {
    match ARGS.command {
        Command::Local => {
            let client = Client::new(
                &OPTIONS.client_options.host, OPTIONS.client_options.port,
                &OPTIONS.client_options.server, 0,
                OPTIONS.client_options.secret.as_deref()
            ).await.unwrap_or_else(|err| {
                eprintln!("{RED}{BOLD} ! {C_RESET} {err}");
                std::process::exit(1);
            });

            client.listen().await.unwrap_or_else(|err| {
                eprintln!("{RED}{BOLD} ! {C_RESET} {err}")
            });
        },
        Command::Server => {
            let port_range = OPTIONS.server_options.min_port..=OPTIONS.server_options.max_port;
            if port_range.is_empty() {
                eprintln!("{RED}{BOLD} ! {RESET} Port range is empty{C_RESET}");
            }
            Server::new(port_range, OPTIONS.server_options.secret.as_deref()).listen().await?;
        }
        Command::Auth => {
            auth(Auth::strawberry_id()).await?;
        }
        Command::About => commands::about::about(),
        Command::None => commands::help::help(),
    }

    Ok(())
}
