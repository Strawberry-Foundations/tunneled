use std::fs;
use serde::{Deserialize, Serialize};

use stblib::colors::{BLUE, BOLD, C_RESET, GREEN, RED, RESET, YELLOW};

use crate::auth::strawberry_id::StrawberryId;
use crate::statics::STRAWBERRY_ID_API;

#[derive(Debug, Serialize, Deserialize)]
struct Credentials {
    username: String,
    token: String,
}

pub async fn auth(mut auth: StrawberryId) -> anyhow::Result<()> {
    println!("{BOLD}{GREEN}--- Strawberry ID Login ---{C_RESET}");

    let request = reqwest::get(format!("{STRAWBERRY_ID_API}api/request")).await?;
    let code = if request.status().is_success() {
        match request.text().await {
            Ok(code) => code,
            Err(err) => {
                eprintln!("{BOLD}{RED} ! {RESET} Error while requesting login code: {err}{C_RESET}");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("{BOLD}{RED} ! {RESET} Error while requesting login code: Server is not reachable{C_RESET}");
        std::process::exit(1);
    };

    // println!("Go to {BOLD}{BLUE}{STRAWBERRY_ID_API}de/login?oauth=true&service=tunneled{C_RESET} and enter the following code: {BOLD}{CYAN}{code}{C_RESET}");
    println!("Go to {BOLD}{BLUE}{STRAWBERRY_ID_API}de/login/oauth_dialog/tunneled?code={code}{C_RESET}");

    let credentials = auth.login(code).await?;

    println!("{GREEN}{BOLD}Logged in as {} (@{})", credentials.full_name, credentials.username);

    if let Some(home_dir) = dirs::home_dir() {
        let config_dir = home_dir.join(".config").join("tunneled");
        let credentials_path = config_dir.join("credentials.yml");

        if !config_dir.exists() {
            if let Err(err) = fs::create_dir_all(&config_dir) {
                eprintln!("{RED}{BOLD}Error while creating config directory:{RESET} {}{C_RESET}", err);
            }
        }

        if !credentials_path.exists() {
            let credentials = Credentials {
                username: credentials.username.clone(),
                token: credentials.token.clone(),
            };

            match serde_yaml::to_string(&credentials) {
                Ok(credentials_str) => {
                    if let Err(err) = fs::write(&credentials_path, credentials_str) {
                        eprintln!("{RED}{BOLD}Error while writing file:{RESET} {}{C_RESET}", err);
                    } else {
                        println!("{GREEN}{BOLD}Credentials saved successfully to {:?}{C_RESET}", credentials_path);
                    }
                }
                Err(err) => eprintln!("{RED}{BOLD}Error while serializing data:{RESET} {}{C_RESET}", err),
            }
        } else {
            println!("{YELLOW}{BOLD}credentials.yml already exists at {:?}{C_RESET}", credentials_path);
        }

    } else {
        eprintln!("{RED}{BOLD}Error while creating config directory:{RESET} Home directory not found.{C_RESET}");
    }

    Ok(())
}