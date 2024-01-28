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
    pub token: String,
}

pub struct StrawberryIdAuthenticator {
    pub username: String,
    pub token: String,
}

pub struct Auth;

impl Auth {
    pub fn strawberry_id() -> StrawberryId {
        StrawberryId {
            email: String::new(),
            full_name: String::new(),
            profile_picture: String::new(),
            username: String::new(),
            token: String::new(),
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
                    self.token = data["data"]["token"].as_str().unwrap().to_string();

                    break
                }
            }

            interval.tick().await;
        }

        Ok(self)
    }

    pub fn authenticator(username: String, token: String) -> (Self, StrawberryIdAuthenticator) {
        (
            Self {
                email: "".to_string(),
                full_name: "".to_string(),
                profile_picture: "".to_string(),
                username: username.clone(),
                token: token.clone(),
        },
            StrawberryIdAuthenticator {
                username,
                token,
            }
        )
    }
}

impl StrawberryIdAuthenticator {
    fn serializer(&self, text: &str) -> Result<Value, serde_json::Error> {
        let serializer = serde_json::from_str(text)?;
        Ok(serializer)
    }

    pub async fn check_id(&mut self, mut strawberry_id: StrawberryId) -> anyhow::Result<(bool, &Self, StrawberryId)> {
        let auth = reqwest::get(format!("{STRAWBERRY_ID_API}api/auth?username={}&token={}", self.username.clone(), self.token.clone())).await?;
        let body = auth.text().await?;

        if let Ok(data) = self.serializer(body.as_str()) {
            if data["data"] != "Invalid token" && data["data"] != "Invalid username" {
                strawberry_id.full_name = data["data"]["full_name"].as_str().unwrap().to_string();
                strawberry_id.email = data["data"]["email"].as_str().unwrap().to_string();
                strawberry_id.profile_picture = data["data"]["profile_picture_url"].as_str().unwrap().to_string();
                strawberry_id.username = data["data"]["username"].as_str().unwrap().to_string();

                return Ok((true, self, strawberry_id));
            }
        }

        Ok((false, self, strawberry_id))
    }
}