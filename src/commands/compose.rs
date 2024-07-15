use std::sync::Arc;
use std::fs::File;
use std::io::Read;

use anyhow::{bail, Context, Result};
use tokio::{io::AsyncWriteExt, net::TcpStream, time::timeout};
use tracing::{error, info, info_span, warn, Instrument};
use uuid::Uuid;
use serde::Deserialize;
use stblib::colors::{BOLD, C_RESET, CYAN, RED, RESET};

use crate::auth::authenticator::StrawberryIdAuthenticator;
use crate::auth::secret::Authenticator;
use crate::shared::{proxy, ClientMessage, Delimited, ServerMessage, NETWORK_TIMEOUT};
use crate::cli::OPTIONS;
use crate::statics::{LOGGER, LOGGER_2};


#[derive(Debug, Deserialize, Clone)]
pub struct Service {
    pub name: String,
    pub host: Option<String>,
    pub server: Option<String>,
    pub secret: Option<String>,
    pub port: u16,
    #[serde(rename = "static-port")]
    pub static_port: Option<u16>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Services {
    pub services: Vec<Service>,
}

pub fn read_service_file(file_path: &str) -> Result<Services, Box<dyn std::error::Error>> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("{RED}{BOLD} ! {RESET} File '{CYAN}{file_path}{RESET}' not found{C_RESET}");
            std::process::exit(1);
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let services: Services = serde_yaml::from_str(&contents)?;
    Ok(services)
}


pub async fn compose(path: Option<&str>) -> Result<()> {
    let path = path.unwrap_or("services.yml");
    let services = read_service_file(path).unwrap();
    let mut handles = vec![];


    for service in services.services.clone() {
        let handle = tokio::spawn(async move {
            let client = Client::new(
                &service.host.unwrap_or(String::from("localhost")),
                service.port,
                &service.server.unwrap_or(String::from("strawberryfoundations.org")),
                service.secret.as_deref()
            ).await.unwrap_or_else(|err| {
                eprintln!("{RED}{BOLD} ! {C_RESET} {err}");
                std::process::exit(1);
            });

            client.listen().await.unwrap_or_else(|err| {
                eprintln!("{RED}{BOLD} ! {C_RESET} {err}")
            });
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}


/// State structure for the client.
pub struct Client {
    /// Control connection to the server.
    conn: Option<Delimited<TcpStream>>,

    /// Destination address of the server.
    to: String,

    // Local host that is forwarded.
    local_host: String,

    /// Local port that is forwarded.
    local_port: u16,

    /// Optional secret used to authenticate clients.
    auth: Option<Authenticator>,
}

impl Client {
    /// Create a new client.
    pub async fn new(
        host: &str,
        port: u16,
        server: &str,
        secret: Option<&str>
    ) -> Result<Self> {
        let mut stream = Delimited::new(connect_with_timeout(server, OPTIONS.client_options.control_port).await.unwrap_or_else(|err| {
            eprintln!(" {RED}{BOLD}!{C_RESET}  Server Error: {err}");
            std::process::exit(1)
        }));

        let auth = secret.map(Authenticator::new);

        if let Some(auth) = &auth {
            auth.client_handshake(&mut stream).await.unwrap();
        }

        let id = if OPTIONS.client_options.auth {
            match StrawberryIdAuthenticator::fetch() {
                Ok(id) => Some(id),
                Err(_) => None
            }
        }
        else {
            None
        };

        stream.send(ClientMessage::Hello(0, id, OPTIONS.client_options.static_port)).await.unwrap();

        let remote_port = match stream.recv_timeout().await.unwrap() {
            Some(ServerMessage::Hello(remote_port)) => remote_port,
            Some(ServerMessage::Error(message)) => bail!("Server Error: {message}"),
            Some(ServerMessage::Challenge(_)) => bail!("Server Error: Server requires authentication, but no client secret was provided"),
            Some(_) => bail!("Server Error: unexpected initial non-hello message"),
            None => bail!("Server Error: unexpected EOF"),
        };

        LOGGER.default(format!("Starting tunneling for {host}:{port}->{server}"));

        if OPTIONS.client_options.auth {
            LOGGER_2.info("Using Strawberry ID Authentication");
        }


        LOGGER.info(format!("Connected to server {server}"));
        LOGGER.info(format!("Listening at {server}:{remote_port}"));

        Ok(Client {
            conn: Some(stream),
            to: server.to_string(),
            local_host: host.to_string(),
            local_port: port,
            auth,
        })
    }

    /// Returns the port publicly available on the remote.
    /* pub fn remote_port(&self) -> u16 {
        self.remote_port
    } */

    /// Start the client, listening for new connections.
    pub async fn listen(mut self) -> Result<()> {
        let mut conn = self.conn.take().unwrap();
        let this = Arc::new(self);
        loop {
            match conn.recv().await? {
                Some(ServerMessage::Hello(_)) => warn!("unexpected hello"),
                Some(ServerMessage::Challenge(_)) => warn!("unexpected challenge"),
                Some(ServerMessage::Heartbeat) => (),
                Some(ServerMessage::Connection(id)) => {
                    let this = Arc::clone(&this);
                    tokio::spawn(
                        async move {
                            info!("new connection");
                            match this.handle_connection(id).await {
                                Ok(_) => info!("connection exited"),
                                Err(err) => warn!(%err, "connection exited with error"),
                            }
                        }
                            .instrument(info_span!("proxy", %id)),
                    );
                }
                Some(ServerMessage::Error(err)) => error!(%err, "server error"),
                None => return Ok(()),
            }
        }
    }

    async fn handle_connection(&self, id: Uuid) -> Result<()> {
        let mut remote_conn =
            Delimited::new(connect_with_timeout(&self.to[..], OPTIONS.client_options.control_port).await?);
        if let Some(auth) = &self.auth {
            auth.client_handshake(&mut remote_conn).await?;
        }
        remote_conn.send(ClientMessage::Accept(id)).await?;
        let mut local_conn = connect_with_timeout(&self.local_host, self.local_port).await?;
        let parts = remote_conn.into_parts();
        debug_assert!(parts.write_buf.is_empty(), "framed write buffer not empty");
        local_conn.write_all(&parts.read_buf).await?; // mostly of the cases, this will be empty
        proxy(local_conn, parts.io).await?;
        Ok(())
    }
}

async fn connect_with_timeout(to: &str, port: u16) -> Result<TcpStream> {
    match timeout(NETWORK_TIMEOUT, TcpStream::connect((to, port))).await {
        Ok(res) => res,
        Err(err) => Err(err.into()),
    }
        .with_context(|| format!("could not connect to {to}:{port}"))
}