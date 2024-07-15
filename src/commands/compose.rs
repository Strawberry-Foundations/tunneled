use std::fs::File;
use std::io::Read;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Service {
    pub name: String,
    pub port: u16,
    #[serde(rename = "static-port")]
    pub static_port: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct Services {
    pub services: Vec<Service>,
}

pub fn read_service_file(file_path: &str) -> Result<Services, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let services: Services = serde_yaml::from_str(&contents)?;
    Ok(services)
}


pub fn compose(path: Option<&str>) -> anyhow::Result<()> {
    let path = path.unwrap_or("services.yml");
    let services = read_service_file(path).unwrap();
    
    println!("{services:?}");

    Ok(())
}