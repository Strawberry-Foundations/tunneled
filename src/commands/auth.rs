use stblib::colors::{BLUE, BOLD, C_RESET, CYAN, GREEN, RED, RESET};
use crate::auth::strawberry_id::StrawberryId;
use crate::statics::STRAWBERRY_ID_API;

pub async fn auth(mut auth: StrawberryId) -> anyhow::Result<()> {
    println!("{BOLD}{GREEN}--- Strawberry ID Login ---{C_RESET}");

    let request = reqwest::get(format!("{STRAWBERRY_ID_API}api/request_code")).await?;
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

    println!("Go to {BOLD}{BLUE}{STRAWBERRY_ID_API}de/oauth/remote?service=tunneled{C_RESET} and enter the following code: {BOLD}{CYAN}{code}{C_RESET}");

    let credentials = auth.login(code).await?;

    println!("{GREEN}{BOLD}Logged in as {} (@{})", credentials.full_name, credentials.username);

    Ok(())
}