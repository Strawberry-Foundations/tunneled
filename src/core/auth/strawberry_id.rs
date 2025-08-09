#![allow(dead_code)]

use libstrawberry::colors::{BOLD, C_RESET, GREEN};
use serde_json::Value;
use tokio::time::{self, Duration};

use crate::core::auth::authenticator::StrawberryIdAuthenticator;
use crate::core::constants::STRAWBERRY_ID_API;

#[derive(Debug, Default, Clone)]
pub struct StrawberryId {
    pub email: String,
    pub full_name: String,
    pub profile_picture: String,
    pub username: String,
    pub token: String,
}

impl StrawberryId {
    fn serializer(&self, text: &str) -> Result<Value, serde_json::Error> {
        let serializer = serde_json::from_str(text)?;
        Ok(serializer)
    }

    pub async fn login(&mut self, code: String) -> anyhow::Result<&Self> {
        let mut interval = time::interval(Duration::from_secs(5));

        loop {
            let response =
                reqwest::get(format!("{STRAWBERRY_ID_API}api/oauth/callback?code={code}")).await?;
            let body = response.text().await?;

            if let Ok(data) = self.serializer(body.as_str())
                && data["data"]["status"] != "Invalid Code"
                && data["data"]["status"] != "Not authenticated"
            {
                println!("{GREEN}{BOLD}Authentication successful{C_RESET}");

                self.email = data["data"]["user"]["email"].as_str().unwrap().to_string();
                self.full_name = data["data"]["user"]["full_name"]
                    .as_str()
                    .unwrap()
                    .to_string();
                self.profile_picture = data["data"]["user"]["profile_picture_url"]
                    .as_str()
                    .unwrap()
                    .to_string();
                self.username = data["data"]["user"]["username"]
                    .as_str()
                    .unwrap()
                    .to_string();
                self.token = data["data"]["user"]["token"].as_str().unwrap().to_string();

                break;
            }

            interval.tick().await;
        }

        Ok(self)
    }

    pub fn authenticator(username: String, token: String) -> StrawberryIdAuthenticator {
        StrawberryIdAuthenticator {
            username: Some(username),
            token: Some(token),
        }
    }
}
