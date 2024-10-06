use anyhow::Result;
use serde::Deserialize;
use stblib::colors::{BOLD, CYAN, C_RESET, RED, RESET};
use std::fs::File;
use std::io::Read;

use crate::commands::client::Client;

#[derive(Debug, Deserialize, Clone)]
pub struct Service {
    pub name: String,
    pub port: u16,
    pub host: Option<String>,
    pub server: Option<String>,
    pub secret: Option<String>,
    #[serde(rename = "static-port")]
    pub static_port: Option<u16>,
    #[serde(rename = "control-port")]
    pub control_port: Option<u16>,
    #[serde(rename = "use-auth")]
    pub use_auth: Option<bool>,
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
            let service_clone = service.clone();

            let client = Client::new(
                &service.host.unwrap_or(String::from("localhost")),
                service.port,
                &service
                    .server
                    .unwrap_or(String::from("strawberryfoundations.org")),
                service.secret.as_deref(),
                service.static_port,
                service.control_port.unwrap_or(7835),
                service.use_auth.unwrap_or(false),
                Option::from(&service_clone),
            )
            .await
            .unwrap_or_else(|err| {
                eprintln!("{RED}{BOLD} ! {C_RESET} {err}");
                std::process::exit(1);
            });

            client
                .listen()
                .await
                .unwrap_or_else(|err| eprintln!("{RED}{BOLD} ! {C_RESET} {err}"));
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}
