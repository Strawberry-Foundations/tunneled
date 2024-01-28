use std::fs;
use serde::{Deserialize, Serialize};
use stblib::colors::{BOLD, C_RESET, RED, RESET};

#[derive(Debug, Serialize, Deserialize)]
pub struct IdAuth {
    pub username: String,
    pub token: String,
}

impl IdAuth {
    pub fn new() -> Result<IdAuth, Box<dyn std::error::Error>> {
        if let Some(home_dir) = dirs::home_dir() {
            let config_dir = home_dir.join(".config").join("tunneled");
            let credentials_path = config_dir.join("credentials.yml");

            if credentials_path.exists() {
                let credentials_str = fs::read_to_string(&credentials_path)?;

                let credentials: IdAuth = serde_yaml::from_str(&credentials_str)?;

                Ok(credentials)
            } else {
                Err(format!("{RED}{BOLD}Error while reading credentials:{RESET} credentials.yml does not exist. Please run tunneled auth to authenticate your Strawberry ID.{C_RESET}").into())
            }
        } else {
            Err(format!("{RED}{BOLD}Error while reading credentials:{RESET} Home directory not found.{C_RESET}").into())
        }
    }
}