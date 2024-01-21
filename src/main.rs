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

use tunneled::client::Client;
use tunneled::server::Server;

use crate::cli::{ARGS, OPTIONS};
use crate::cli::args::Command;
use crate::auth::strawberry_id::Auth;

mod cli;
mod commands;
mod statics;
mod auth;

#[tokio::main]
async fn main() -> Result <()> {
    match ARGS.command {
        Command::Local => {
            let client = Client::new(
                &OPTIONS.client_options.host, OPTIONS.client_options.port,
                &OPTIONS.client_options.server, 0,
                OPTIONS.client_options.secret.as_deref()
            ).await.unwrap();

            client.listen().await.unwrap();
        },
        Command::Server => {
            let port_range = OPTIONS.server_options.min_port..=OPTIONS.server_options.max_port;
            if port_range.is_empty() {
                eprintln!("{RED}{BOLD} ! {RESET} Port range is empty{C_RESET}");
            }
            Server::new(port_range, OPTIONS.server_options.secret.as_deref()).listen().await?;
        }
        Command::Auth => {
            commands::auth::auth(Auth::strawberry_id()).await?;



            // let id = auth.to_strawberry_id();
        }
        Command::None => commands::help::help(),
    }

    Ok(())
}
