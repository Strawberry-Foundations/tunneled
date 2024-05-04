//! Server implementation for the `bore` service.

use std::{io, net::SocketAddr, ops::RangeInclusive, sync::Arc, time::Duration};

use anyhow::Result;
use dashmap::DashMap;
use stblib::colors::{BOLD, C_RESET, YELLOW};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, timeout};
use tracing::{info, info_span, warn, Instrument};
use uuid::Uuid;


use crate::auth_2::Authenticator;
use crate::cli::OPTIONS;
use crate::shared::{proxy, ClientMessage, Delimited, ServerMessage};
use crate::statics::{LOGGER, STRAWBERRY_ID_API};

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
        LOGGER.info(format!("Server is listening on {addr}"));

        if OPTIONS.server_options.require_id {
            LOGGER.info(format!("Using Strawberry ID Authentication ({STRAWBERRY_ID_API})"));
        }

        loop {
            let (stream, addr) = listener.accept().await?;
            let this = Arc::clone(&this);
            tokio::spawn(
                async move {
                    LOGGER.info(format!("Incoming connection from {}", addr));

                    if let Err(err) = this.handle_connection(stream).await {
                        LOGGER.warning(format!("Connection exited with error {err}"));
                    } else {
                        LOGGER.info("Connection exited");
                    }
                }
                .instrument(info_span!("control", ?addr)),
            );
        }
    }

    async fn create_listener(&self, port: u16) -> Result<TcpListener, &'static str> {
        let try_bind = |port: u16| async move {
            TcpListener::bind(("0.0.0.0", port))
                .await
                .map_err(|err| match err.kind() {
                    io::ErrorKind::AddrInUse => "port already in use",
                    io::ErrorKind::PermissionDenied => "permission denied",
                    _ => "failed to bind to port",
                })
        };
        if port > 0 {
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
                warn!(%err, "server handshake failed");
                stream.send(ServerMessage::Error(err.to_string())).await?;
                return Ok(());
            }
        }

        match stream.recv_timeout().await? {
            Some(ClientMessage::Authenticate(_)) => {
                warn!("unexpected authenticate");
                Ok(())
            }
            Some(ClientMessage::Hello(port, mut id)) => {
                let listener = match self.create_listener(port).await {
                    Ok(listener) => listener,
                    Err(err) => {
                        stream.send(ServerMessage::Error(err.into())).await?;
                        return Ok(());
                    }
                };

                let port = listener.local_addr()?.port();

                LOGGER.info(format!(" ↳ New Client listening at port {port}"));

                if OPTIONS.server_options.require_id {
                    if id.username.is_none() && id.token.is_none() {
                        LOGGER.info(format!(" ↳ {YELLOW}{BOLD}!{C_RESET} Invalid Strawberry ID Auth (Client connected without Strawberry ID)"));
                        stream.send(ServerMessage::Error(
                            "This server requires a Strawberry ID which you didn't provide. \
                            Please add the --auth Flag (and if not already done, log in with your Strawberry ID with tunneled auth)".to_string()
                        )).await?;
                        
                        return Ok(())
                    }
                    
                    let (username, token) = id.clone().unwrap();
                    
                    LOGGER.info(format!(" ↳ Received Strawberry ID Auth (@{})", username));
                    
                    let auth = id.verify(&username, &token).await?;

                    if let Some(auth) = auth {
                        LOGGER.info(format!(" ↳ Authentication successful ({} (@{}))", auth.strawberry_id.full_name, auth.strawberry_id.username));
                    } else {
                        LOGGER.info(format!(" ↳ {YELLOW}{BOLD}!{C_RESET} Invalid Strawberry ID Auth (@{username})"));
                        stream.send(ServerMessage::Error("Invalid Strawberry ID".to_string())).await?;
                    }
                }

                stream.send(ServerMessage::Hello(port)).await?;

                loop {
                    if stream.send(ServerMessage::Heartbeat).await.is_err() {
                        // Assume that the TCP connection has been dropped.
                        return Ok(());
                    }
                    const TIMEOUT: Duration = Duration::from_millis(500);
                    if let Ok(result) = timeout(TIMEOUT, listener.accept()).await {
                        let (stream2, addr) = result?;
                        LOGGER.info(format!("New Connection at {addr}:{port}"));

                        let id = Uuid::new_v4();
                        let conns = Arc::clone(&self.conns);

                        conns.insert(id, stream2);
                        tokio::spawn(async move {
                            // Remove stale entries to avoid memory leaks.
                            sleep(Duration::from_secs(10)).await;
                            if conns.remove(&id).is_some() {
                                warn!(%id, "removed stale connection");
                            }
                        });
                        stream.send(ServerMessage::Connection(id)).await?;
                    }
                }
            }
            Some(ClientMessage::Accept(id)) => {
                info!(%id, "forwarding connection");
                match self.conns.remove(&id) {
                    Some((_, mut stream2)) => {
                        let parts = stream.into_parts();
                        debug_assert!(parts.write_buf.is_empty(), "framed write buffer not empty");
                        stream2.write_all(&parts.read_buf).await?;
                        proxy(parts.io, stream2).await?
                    }
                    None => warn!(%id, "missing connection"),
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
