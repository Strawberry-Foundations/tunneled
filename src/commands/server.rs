#![allow(unused_assignments)]
//! Server implementation for the `bore` service.

use std::{io, net::SocketAddr, ops::RangeInclusive, sync::Arc, time::Duration};

use anyhow::Result;
use dashmap::DashMap;
use stblib::colors::{BOLD, C_RESET, MAGENTA, RESET, YELLOW};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, timeout};
use tracing::{info, info_span, warn, Instrument};
use uuid::Uuid;

use crate::auth::authenticator::{ClientAuthentication};
use crate::auth::secret::Authenticator;
use crate::cli::OPTIONS;
use crate::shared::{proxy, ClientMessage, Delimited, ServerMessage};
use crate::statics::{LOGGER, LOGGER_2, STRAWBERRY_ID_API, VERSION};

/// State structure for the server.
pub struct Server {
    /// Range of TCP ports that can be forwarded.
    port_range: RangeInclusive<u16>,

    /// Optional secret used to authenticate clients.
    auth: Option<Authenticator>,

    /// Concurrent map of IDs to incoming connections.
    conns: Arc<DashMap<Uuid, TcpStream>>,
}

impl Server {
    /// Create a new server with a specified minimum port number.
    pub fn new(port_range: RangeInclusive<u16>, secret: Option<&str>) -> Self {
        assert!(!port_range.is_empty(), "must provide at least one port");
        Server {
            port_range,
            conns: Arc::new(DashMap::new()),
            auth: secret.map(Authenticator::new),
        }
    }

    /// Start the server, listening for new connections.
    pub async fn listen(self) -> Result<()> {
        let this = Arc::new(self);
        let addr = SocketAddr::from(([0, 0, 0, 0], OPTIONS.server_options.control_port));
        let listener = TcpListener::bind(&addr).await?;

        LOGGER_2.default(format!("Starting Tunneled server v{}", *VERSION));
        LOGGER.info(format!("Server is listening on {addr}"));

        if OPTIONS.server_options.require_id {
            LOGGER_2.info(format!("Using Strawberry ID Authentication ({STRAWBERRY_ID_API})"));
        }

        loop {
            let (stream, addr) = listener.accept().await?;
            let this = Arc::clone(&this);
            tokio::spawn(
                async move {
                    LOGGER.info(format!("[{MAGENTA}{addr}{RESET}] Incoming connection"));

                    if let Err(err) = this.handle_connection(stream).await {
                        LOGGER.warning(format!("[{MAGENTA}{addr}{RESET}] Connection exited with error {err}"));
                    } else {
                        LOGGER.info(format!("[{MAGENTA}{addr}{RESET}] Connection exited"));
                    }
                }
                .instrument(info_span!("control", ?addr)),
            );
        }
    }

    #[allow(unused_assignments)]
    async fn create_listener(&self, port: u16, static_port: Option<u16>, id: &Option<ClientAuthentication>) -> Result<TcpListener, &'static str> {
        let whitelist = ["julian@strawberryfoundations.xyz"];

        let try_bind = |port: u16| async move {
            TcpListener::bind(("0.0.0.0", port))
                .await
                .map_err(|err| match err.kind() {
                    io::ErrorKind::AddrInUse => "port already in use",
                    io::ErrorKind::PermissionDenied => "permission denied",
                    _ => "failed to bind to port",
                })
        };

        if let Some(static_port) = static_port {
            if let Some(id) = id {
                if whitelist.contains(&&*id.strawberry_id.email) {
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

    async fn handle_connection(&self, stream: TcpStream) -> Result<()> {
        let mut stream = Delimited::new(stream);
        if let Some(auth) = &self.auth {
            if let Err(err) = auth.server_handshake(&mut stream).await {
                LOGGER.warning(format!("Server handshake failed ({err}"));
                stream.send(ServerMessage::Error(err.to_string())).await?;
                return Ok(());
            }
        }

        match stream.recv_timeout().await? {
            Some(ClientMessage::Authenticate(_)) => {
                LOGGER.warning("Unexpected authenticate");
                Ok(())
            }
            Some(ClientMessage::Hello(port, id, static_port)) => {
                let strawberry_id = if OPTIONS.server_options.require_id {
                    if let Some(mut id) = id.clone() {
                        let (username, token) = id.clone().unwrap();

                        LOGGER.info(format!(" ↳ Received Strawberry ID Auth (@{})", username));

                        let auth = id.verify(&username, &token).await?;

                        if let Some(auth) = auth.clone() {
                            LOGGER.info(format!(" ↳ Authentication successful ({} (@{}))", auth.strawberry_id.full_name, auth.strawberry_id.username));

                        } else {
                            LOGGER.info(format!(" ↳ {YELLOW}{BOLD}!{C_RESET} Invalid Strawberry ID Auth (@{username})"));
                            stream.send(ServerMessage::Error("Invalid Strawberry ID".to_string())).await?;
                            
                            return Ok(())
                        }
                        
                        auth
                    } else {
                        LOGGER.info(format!(" ↳ {YELLOW}{BOLD}!{C_RESET} Invalid Strawberry ID Auth (Client connected without Strawberry ID)"));
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

                LOGGER.info(format!(" ↳ New client listening at port {port}"));

                stream.send(ServerMessage::Hello(port)).await?;

                loop {
                    if stream.send(ServerMessage::Heartbeat).await.is_err() {
                        // Assume that the TCP connection has been dropped.
                        return Ok(());
                    }
                    const TIMEOUT: Duration = Duration::from_millis(500);
                    if let Ok(result) = timeout(TIMEOUT, listener.accept()).await {
                        let (stream2, addr) = result?;
                        LOGGER.info(format!("New connection at {addr}:{port}"));

                        let id = Uuid::new_v4();
                        let conns = Arc::clone(&self.conns);

                        conns.insert(id, stream2);
                        tokio::spawn(async move {
                            // Remove stale entries to avoid memory leaks.
                            sleep(Duration::from_secs(10)).await;
                            if conns.remove(&id).is_some() {
                                LOGGER.warning(format!("Removed stale connection ({id})"));
                            }
                        });
                        stream.send(ServerMessage::Connection(id)).await?;
                    }
                }
            }
            Some(ClientMessage::Accept(id)) => {
                info!(%id, "Forwarding connection");
                match self.conns.remove(&id) {
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
