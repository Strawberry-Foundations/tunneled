use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use stblib::colors::{BOLD, C_RESET, CYAN, RED, RESET};

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


pub fn compose(path: Option<&str>) -> anyhow::Result<()> {
    let path = path.unwrap_or("services.yml");
    let services = read_service_file(path).unwrap();

    for service in &services.services {

    }

    println!("{services:?}");

    Ok(())
}