#![allow(dead_code)]

use tokio::time::{self, Duration};
use serde_json::Value;
use stblib::colors::{BOLD, C_RESET, GREEN};
use crate::statics::STRAWBERRY_ID_API;

pub struct StrawberryId {
    pub email: String,
    pub full_name: String,
    pub profile_picture: String,
    pub username: String,
}

pub struct Auth;

impl Auth {
    pub fn strawberry_id() -> StrawberryId {
        StrawberryId {
                email: String::new(),
                full_name: String::new(),
                profile_picture: String::new(),
                username: String::new(),
        }
    }
}

impl StrawberryId {
    fn serializer(&self, text: &str) -> Result<Value, serde_json::Error> {
        let serializer = serde_json::from_str(text)?;
        Ok(serializer)
    }

    pub async fn login(&mut self, code: String) -> anyhow::Result<&Self> {
        let mut interval = time::interval(Duration::from_secs(5));

        loop {
            let response = reqwest::get(format!("{STRAWBERRY_ID_API}api/oauth_validate?code={code}")).await?;
            let body = response.text().await?;

            if let Ok(data) = self.serializer(body.as_str()) {
                if data["data"] != "Invalid Code" && data["data"] != "Not authenticated" {
                    println!("{GREEN}{BOLD}Authentication successful (Strawberry ID){C_RESET}");

                    self.email = data["data"]["email"].as_str().unwrap().to_string();
                    self.full_name = data["data"]["full_name"].as_str().unwrap().to_string();
                    self.profile_picture = data["data"]["profile_picture_url"].as_str().unwrap().to_string();
                    self.username = data["data"]["username"].as_str().unwrap().to_string();

                    break
                }
            }

            interval.tick().await;
        }

        Ok(self)
    }
}