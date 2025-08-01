#![allow(clippy::too_many_arguments)]
//! Client implementation for the `tunneled` service.

use std::sync::Arc;

use anyhow::{Context, Result, bail};
use libstrawberry::colors::{BLUE, BOLD, C_RESET, CYAN, GRAY, ITALIC, MAGENTA, RED, RESET};
use tokio::{io::AsyncWriteExt, net::TcpStream, time::timeout};
use tracing::{Instrument, info_span};
use uuid::Uuid;

use crate::cli::OPTIONS;
use crate::commands::compose::Service;
use crate::core::auth::authenticator::StrawberryIdAuthenticator;
use crate::core::auth::secret::Authenticator;
use crate::core::constants::{SERVER_LOG, CLIENT_LOG};
use crate::core::shared::{ClientMessage, Delimited, NETWORK_TIMEOUT, ServerMessage};

/// State structure for the client.
pub struct Client {
    /// Control connection to the server.
    connection: Option<Delimited<TcpStream>>,

    /// Destination address of the server.
    to: String,

    /// Local host that is forwarded.
    local_host: String,

    /// Local port that is forwarded.
    local_port: u16,

    /// Tcp connection port for remote server
    control_port: u16,

    /// Optional secret used to authenticate clients.
    auth: Option<Authenticator>,
}

impl Client {
    /// Create a new client.
    pub async fn new(
        host: &str,
        port: u16,
        server: &str,
        secret: Option<&str>,
        static_port: Option<u16>,
        control_port: u16,
        require_auth: bool,
        service: Option<&Service>,
    ) -> Result<Self> {
        let mut stream = Delimited::new(
            connect_with_timeout(server, control_port)
                .await
                .unwrap_or_else(|err| {
                    eprintln!(" {RED}{BOLD}!{C_RESET}  Server Error: {err}");
                    std::process::exit(1)
                }),
        );

        let auth = secret.map(Authenticator::new);

        if let Some(auth) = &auth {
            auth.client_handshake(&mut stream).await?;
        }

        let id = if require_auth {
            StrawberryIdAuthenticator::fetch().ok()
        } else {
            None
        };

        stream
            .send(ClientMessage::Hello(0, id, static_port))
            .await?;

        let (addr, remote_port) = match stream.recv_timeout().await? {
            Some(ServerMessage::Hello(addr, remote_port)) => (addr, remote_port),
            Some(ServerMessage::Error(message)) => bail!("Server Error: {message}"),
            Some(ServerMessage::Challenge(_)) => bail!(
                "Server Error: Server requires authentication, but no client secret was provided"
            ),
            Some(_) => bail!("Server Error: unexpected initial non-hello message"),
            None => bail!("Server Error: unexpected EOF"),
        };

        if let Some(service) = service {
            CLIENT_LOG.ok(format!(
                "Starting tunneling service '{CYAN}{}{RESET}'",
                service.name
            ));
            CLIENT_LOG.info(format!(
                "Forwarding rule: {BLUE}{host}:{port}{RESET}->{ITALIC}{MAGENTA}{server}{RESET}"
            ));
        }

        if service.is_none() {
            CLIENT_LOG.ok(format!("Starting tunneling for {BLUE}{host}:{port}{RESET}->{ITALIC}{MAGENTA}{server}{RESET}"));
        }

        if require_auth {
            CLIENT_LOG.info("Using Strawberry ID Authentication");
        }

        SERVER_LOG.info(format!(
            "Connected to server {MAGENTA}{ITALIC}{server}{C_RESET}"
        ));
        SERVER_LOG.info(format!("Listening at {BLUE}{addr}:{remote_port}{RESET}"));

        if service.is_some() {
            println!()
        }

        Ok(Client {
            connection: Some(stream),
            to: server.to_string(),
            local_host: host.to_string(),
            local_port: port,
            control_port,
            auth,
        })
    }

    /// Start the client, listening for new connections.
    pub async fn listen(mut self) -> Result<()> {
        let control_port = self.control_port;
        let mut conn = self.connection.take().unwrap();
        let this = Arc::new(self);
        loop {
            match conn.recv().await? {
                Some(ServerMessage::Hello(_, _)) => SERVER_LOG.warning("Unexpected hello"),
                Some(ServerMessage::Challenge(_)) => SERVER_LOG.warning("Unexpected challenge"),
                Some(ServerMessage::Heartbeat) => (),
                Some(ServerMessage::Connection(id)) => {
                    let this = Arc::clone(&this);
                    tokio::spawn(
                        async move {
                            if OPTIONS.client_options.verbose_logging {
                                SERVER_LOG.info(format!("New connection ({GRAY}{id}{C_RESET})"));    
                            }
                            match this.handle_connection(id, control_port).await {
                                Ok(_) => if OPTIONS.client_options.verbose_logging {
                                    SERVER_LOG.info(format!("Connection exited ({GRAY}{id}{C_RESET})"))
                                },
                                Err(err) => if OPTIONS.client_options.verbose_logging {
                                    SERVER_LOG.error(format!("Connection ({GRAY}{id}{C_RESET}) exited with error: {err}"))
                                },
                            }
                        }.instrument(info_span!("proxy", %id)),
                    );
                }
                Some(ServerMessage::Error(err)) => SERVER_LOG.error(format!("Server error: {err}")),
                None => {
                    CLIENT_LOG.error("Lost connection to tunneled instance");
                    return Ok(());
                }
            }
        }
    }

    async fn handle_connection(&self, id: Uuid, control_port: u16) -> Result<()> {
        let mut remote_conn =
            Delimited::new(connect_with_timeout(&self.to[..], control_port).await?);

        if let Some(auth) = &self.auth {
            auth.client_handshake(&mut remote_conn).await?;
        }

        remote_conn.send(ClientMessage::Accept(id)).await?;
        let mut local_conn = connect_with_timeout(&self.local_host, self.local_port).await?;
        let mut parts = remote_conn.into_parts();

        debug_assert!(parts.write_buf.is_empty(), "framed write buffer not empty");
        local_conn.write_all(&parts.read_buf).await?; // mostly of the cases, this will be empty
        tokio::io::copy_bidirectional(&mut local_conn, &mut parts.io).await?;
        Ok(())
    }
}

pub async fn connect_with_timeout(to: &str, port: u16) -> Result<TcpStream> {
    match timeout(NETWORK_TIMEOUT, TcpStream::connect((to, port))).await {
        Ok(res) => res,
        Err(err) => Err(err.into()),
    }
    .with_context(|| format!("Could not connect to {to}:{port}"))
}
