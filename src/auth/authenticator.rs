use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use stblib::colors::{BOLD, C_RESET, RED, RESET};

use crate::auth::strawberry_id::StrawberryId;
use crate::statics::STRAWBERRY_ID_API;

#[derive(Debug, Default, Clone)]
pub struct ClientAuthentication {
    pub status: bool,
    pub credentials: StrawberryIdAuthenticator,
    pub strawberry_id: StrawberryId,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StrawberryIdAuthenticator {
    pub username: Option<String>,
    pub token: Option<String>,
}

impl StrawberryIdAuthenticator {
    pub fn fetch() -> Result<StrawberryIdAuthenticator, Box<dyn std::error::Error>> {
        if let Some(home_dir) = dirs::home_dir() {
            let config_dir = home_dir.join(".config").join("tunneled");
            let credentials_path = config_dir.join("credentials.yml");

            if credentials_path.exists() {
                let credentials_str = fs::read_to_string(&credentials_path)?;

                let credentials: StrawberryIdAuthenticator = serde_yaml::from_str(&credentials_str)?;

                Ok(credentials)
            } else {
                Err(format!("{RED}{BOLD}Error while reading credentials:{RESET} credentials.yml does not exist. Please run tunneled auth to authenticate your Strawberry ID.{C_RESET}").into())
            }
        } else {
            Err(format!("{RED}{BOLD}Error while reading credentials:{RESET} Home directory not found.{C_RESET}").into())
        }
    }

    pub fn unwrap(self) -> (String, String) {
        (self.username.clone().unwrap(), self.token.unwrap())
    }

    fn serializer(&self, text: &str) -> Result<Value, serde_json::Error> {
        let serializer = serde_json::from_str(text)?;
        Ok(serializer)
    }

    pub async fn verify(&mut self, username: &String, token: &String) -> anyhow::Result<Option<ClientAuthentication>> {
        let auth = reqwest::get(format!("{STRAWBERRY_ID_API}api/auth?username={}&token={}", username, token)).await?;
        let body = auth.text().await?;
        
        let mut client_auth = ClientAuthentication::default();

        if let Ok(data) = self.serializer(body.as_str()) {
            if data["data"]["status"] == "Ok" {
                client_auth.strawberry_id.full_name = data["data"]["user"]["full_name"].as_str().unwrap().to_string();
                client_auth.strawberry_id.email = data["data"]["user"]["email"].as_str().unwrap().to_string();
                client_auth.strawberry_id.profile_picture = data["data"]["user"]["profile_picture_url"].as_str().unwrap().to_string();
                client_auth.strawberry_id.username = data["data"]["user"]["username"].as_str().unwrap().to_string();

                return Ok(Some(client_auth))
            }
        }

        Ok(None)
    }
}