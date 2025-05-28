#![allow(unused_assignments)]
//! Server implementation for the `tunneled` service.

use std::{io, net::SocketAddr, ops::RangeInclusive, sync::Arc, time::Duration};
use std::fs::File;
use std::io::Read;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, timeout};

use anyhow::Result;
use dashmap::DashMap;
use serde::Deserialize;
use stblib::colors::{BOLD, C_RESET, CYAN, MAGENTA, RED, RESET, YELLOW, BLUE, GREEN, ITALIC};
use tracing::{info, info_span, Instrument};
use uuid::Uuid;

use crate::auth::authenticator::{ClientAuthentication};
use crate::auth::secret::Authenticator;
use crate::cli::OPTIONS;
use crate::shared::{proxy, ClientMessage, Delimited, ServerMessage};
use crate::constants::{LOGGER, LOGGER_2, STRAWBERRY_ID_API, VERSION};

/// State structure for the server.
pub struct Server {
    /// Range of TCP ports that can be forwarded.
    port_range: RangeInclusive<u16>,

    /// Optional secret used to authenticate clients.
    auth: Option<Authenticator>,

    /// Concurrent map of IDs to incoming connections.
    connections: Arc<DashMap<Uuid, TcpStream>>,

    /// Access port for tunneled
    control_port: u16,

    /// Require Strawberry ID?
    require_id: bool,

    /// Whitelist for static port users
    whitelist_static_port: Vec<String>,

    /// IP address where the tunneles will listen on
    tunnels_addr: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerHostConfig {
    #[serde(rename = "min-port")]
    pub min_port: u16,
    #[serde(rename = "max-port")]
    pub max_port: u16,
    #[serde(rename = "control-port")]
    pub control_port: Option<u16>,
    #[serde(rename = "tunnels-addr")]
    pub tunnels_addr: Option<String>
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerAuthConfig {
    pub secret: Option<String>,
    #[serde(rename = "require-id")]
    pub require_id: Option<bool>,
    #[serde(rename = "allow-static-port")]
    pub allow_static_port: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSecurityConfig {
    #[serde(rename = "ip-blacklist")]
    pub ip_blacklist: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host: ServerHostConfig,
    pub auth: ServerAuthConfig,
    pub security: ServerSecurityConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub server: Config
}

pub fn read_config_file(file_path: &str) -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("{RED}{BOLD} ! {RESET} File '{CYAN}{file_path}{RESET}' not found{C_RESET}");
            std::process::exit(1);
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let services: ServerConfig = serde_yaml::from_str(&contents)?;
    Ok(services)
}

impl Server {
    /// Create a new server with a specified minimum port number.
    pub fn new(
        port_range: RangeInclusive<u16>,
        secret: Option<&str>,
        control_port: u16,
        require_id: bool,
        whitelist: Vec<String>,
        tunnels_addr: String,
    ) -> Self {
        assert!(!port_range.is_empty(), "must provide at least one port");
        Server {
            port_range,
            connections: Arc::new(DashMap::new()),
            auth: secret.map(Authenticator::new),
            control_port,
            require_id,
            whitelist_static_port: whitelist,
            tunnels_addr
        }
    }

    /// Start the server, listening for new connections.
    pub async fn listen(self) -> Result<()> {
        let this = Arc::new(self);
        let addr = SocketAddr::from(([0, 0, 0, 0], this.control_port));
        let listener = TcpListener::bind(&addr).await?;

        LOGGER_2.default(format!("Starting Tunneled server v{}", *VERSION));
        LOGGER.info(format!("Server is listening on {MAGENTA}{addr}{C_RESET}"));
        LOGGER.info(format!("Port range: {MAGENTA}{}-{}{C_RESET}", this.port_range.start(), this.port_range.end()));
        LOGGER.info(format!("Tunneling address: {MAGENTA}{}{C_RESET}", this.tunnels_addr));

        if OPTIONS.server_options.verbose_logging {
            LOGGER.info(format!("Port range: {MAGENTA}{}-{}{C_RESET}", this.port_range.start(), this.port_range.end()));
            LOGGER.info(format!("Control port: {MAGENTA}{}{C_RESET}", this.control_port));
        }

        if this.require_id {
            LOGGER_2.info(format!("Using Strawberry ID Authentication ({STRAWBERRY_ID_API})"));
        }
        else if this.auth.is_some() {
            LOGGER_2.info("Using secret authentication");
        }
        else {
            LOGGER_2.info("No authentication");
        }


        loop {
            let (stream, addr) = listener.accept().await?;
            let this = Arc::clone(&this);
            tokio::spawn(
                async move {
                    if OPTIONS.server_options.verbose_logging {
                        LOGGER.info(format!("[{MAGENTA}{addr}{RESET}] Incoming connection"));
                    }

                    if let Err(err) = this.handle_connection(stream, &addr).await {
                        LOGGER.warning(format!("[{MAGENTA}{addr}{RESET}] Connection exited with error {err}"));
                    } else if OPTIONS.server_options.verbose_logging {
                        LOGGER.info(format!("[{MAGENTA}{addr}{RESET}] Connection exited"));
                    }   
                }
                .instrument(info_span!("control", ?addr)),
            );
        }
    }

    #[allow(unused_assignments)]
    async fn create_listener(&self, port: u16, static_port: Option<u16>, id: &Option<ClientAuthentication>) -> Result<TcpListener, &'static str> {
        let try_bind = |port: u16| async move {
            TcpListener::bind((self.tunnels_addr.as_ref(), port))
                .await
                .map_err(|err| match err.kind() {
                    io::ErrorKind::AddrInUse => "Port already in use",
                    io::ErrorKind::PermissionDenied => "Permission denied",
                    _ => "Failed to bind to port",
                })
        };

        if let Some(static_port) = static_port {
            if let Some(id) = id {
                if self.whitelist_static_port.contains(&id.strawberry_id.email.to_string()) {
                    match try_bind(static_port).await {
                        Ok(listener) => Ok(listener),
                        Err(err) => {
                            LOGGER.error(format!("Failed to bind to port: {err}"));

                            Err("Port is not available")
                        },
                    }
                }
                else {
                    Err("You are not allowed to use static ports")
                }
            }
            else {
                Err("This feature is currently only available to whitelisted Strawberry ID users")
            }
        }
        else if port > 0 {
            // Client requests a specific port number.
            if !self.port_range.contains(&port) {
                return Err("client port number not in allowed range");
            }
            try_bind(port).await
        } else {
            // Client requests any available port in range.
            //
            // In this case, we bind to 150 random port numbers. We choose this value because in
            // order to find a free port with probability at least 1-δ, when ε proportion of the
            // ports are currently available, it suffices to check approximately -2 ln(δ) / ε
            // independently and uniformly chosen ports (up to a second-order term in ε).
            //
            // Checking 150 times gives us 99.999% success at utilizing 85% of ports under these
            // conditions, when ε=0.15 and δ=0.00001.
            for _ in 0..150 {
                let port = fastrand::u16(self.port_range.clone());
                match try_bind(port).await {
                    Ok(listener) => return Ok(listener),
                    Err(_) => continue,
                }
            }
            Err("failed to find an available port")
        }
    }

    async fn handle_connection(&self, stream: TcpStream, addr: &SocketAddr) -> Result<()> {
        let mut stream = Delimited::new(stream);
        if let Some(auth) = &self.auth {
            if let Err(err) = auth.server_handshake(&mut stream).await {
                LOGGER.warning("Server handshake failed".to_string());
                stream.send(ServerMessage::Error(format!("Handshake failed - {err}"))).await?;
                return Ok(());
            }
        }

        match stream.recv_timeout().await? {
            Some(ClientMessage::Authenticate(_)) => {
                LOGGER.warning("Unexpected authenticate");
                Ok(())
            }
            Some(ClientMessage::Hello(port, id, static_port)) => {
                let strawberry_id = if self.require_id {
                    if let Some(mut id) = id.clone() {
                        let (username, token) = id.clone().unwrap();

                        if OPTIONS.server_options.verbose_logging {
                            LOGGER.info(format!("[{MAGENTA}{addr}{RESET}] Received Strawberry ID Auth (@{username})"));
                        }

                        let auth = id.verify(&username, &token).await?;

                        if let Some(auth) = auth.clone() {
                            LOGGER.info(format!(
                                "[{MAGENTA}{addr}{RESET}] Authentication successful ({GREEN}{}{C_RESET} ({ITALIC}{CYAN}@{}{C_RESET}))", 
                                auth.strawberry_id.full_name, auth.strawberry_id.username
                            ));

                        } else {
                            LOGGER.info(format!("[{MAGENTA}{addr}{RESET}] {YELLOW}{BOLD}<!>{C_RESET} Invalid Strawberry ID Auth (@{username})"));
                            stream.send(ServerMessage::Error("Invalid Strawberry ID".to_string())).await?;
                            
                            return Ok(())
                        }
                        
                        auth
                    } else {
                        LOGGER.info(format!("[{MAGENTA}{addr}{RESET}] {YELLOW}{BOLD}<!>{C_RESET} Invalid Strawberry ID Auth (Client connected without Strawberry ID)"));

                        stream.send(ServerMessage::Error(
                            "This server requires a Strawberry ID which you didn't provide. \
                            Please add the --auth Flag (and if not already done, log in with your Strawberry ID with tunneled auth)".to_string()
                        )).await?;

                        return Ok(());
                    }
                } else {
                    None
                };

                let listener = match self.create_listener(port, static_port, &strawberry_id).await {
                    Ok(listener) => listener,
                    Err(err) => {
                        stream.send(ServerMessage::Error(err.into())).await?;
                        return Ok(());
                    }
                };

                let port = listener.local_addr()?.port();

                LOGGER.info(format!(
                    "[{MAGENTA}{}{C_RESET}] Created tunneling rule for {BLUE}{BOLD}{}{C_RESET}->{MAGENTA}{BOLD}{}:{port}{C_RESET}",
                    addr, addr.ip(), listener.local_addr()?.ip()
                ));

                stream.send(ServerMessage::Hello(port)).await?;

                loop {
                    if stream.send(ServerMessage::Heartbeat).await.is_err() {
                        // Assume that the TCP connection has been dropped.
                        return Ok(());
                    }
                    const TIMEOUT: Duration = Duration::from_millis(500);
                    if let Ok(result) = timeout(TIMEOUT, listener.accept()).await {
                        let (stream2, addr) = result?;

                        if OPTIONS.server_options.verbose_logging {
                            LOGGER.info(format!("External connection at {addr}:{port}"));
                        }

                        let id = Uuid::new_v4();
                        let connections = Arc::clone(&self.connections);

                        connections.insert(id, stream2);
                        tokio::spawn(async move {
                            // Remove stale entries to avoid memory leaks.
                            sleep(Duration::from_secs(10)).await;
                            if connections.remove(&id).is_some() {
                                LOGGER.warning(format!("Removed stale connection ({id})"));
                            }
                        });
                        stream.send(ServerMessage::Connection(id)).await?;
                    }
                }
            }
            Some(ClientMessage::Accept(id)) => {
                info!(%id, "Forwarding connection");
                match self.connections.remove(&id) {
                    Some((_, mut stream2)) => {
                        let parts = stream.into_parts();
                        debug_assert!(parts.write_buf.is_empty(), "Framed write buffer not empty");
                        stream2.write_all(&parts.read_buf).await?;
                        proxy(parts.io, stream2).await?
                    }
                    None => LOGGER.warning(format!("Missing connection ({id})")),
                }
                Ok(())
            }
            None => {
                LOGGER.warning("Client sent empty response");
                Ok(())
            },
        }
    }
}
