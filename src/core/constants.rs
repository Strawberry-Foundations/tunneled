use lazy_static::lazy_static;

use libstrawberry::colors::{BLUE, BOLD, C_RESET, GREEN, RED, YELLOW, GRAY};
use libstrawberry::logging::features::LoggingFeatures;
use libstrawberry::logging::formats::{LogFormat, LogFormatOptions};
use libstrawberry::logging::Logger;

pub const STRAWBERRY_ID_API: &str = "https://id.strawberryfoundations.org/v2/";
// pub const STRAWBERRY_ID_API: &str = "http://192.168.0.194:8082/v1/";

lazy_static! {
    pub static ref VERSION: String = env!("CARGO_PKG_VERSION").to_string();

    pub static ref SERVER_LOG: Logger = Logger::new(
        LoggingFeatures::new(),
        LogFormat {
            info: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {GREEN}[%<levelname>%]{GRAY} @ {GREEN}SERVER{C_RESET}    [%<message>%]"),
            error: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {RED}[%<levelname>%]{GRAY} @ {RED}SERVER{C_RESET}   [%<message>%]"),
            ok: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {BLUE}INIT{C_RESET}{GRAY} @ {BLUE}SERVER{C_RESET}    [%<message>%]"),
            warning: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {YELLOW}WARN{GRAY} @ {YELLOW}SERVER{C_RESET}    [%<message>%]"),
            critical: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {RED}[%<levelname>%]{GRAY} @ {RED}SERVER{C_RESET} [%<message>%]"),
            panic: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {RED}[%<levelname>%]{GRAY} @ {RED}SERVER{C_RESET} [%<message>%]"),
            log_options: LogFormatOptions {
                timestamp_format: "%Y-%m-%d %H:%M:%S".to_string(),
                levelname_lowercase: false
            },
        }
    );

    pub static ref CLIENT_LOG: Logger = Logger::new(
        LoggingFeatures::new(),
        LogFormat {
            info: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {GREEN}[%<levelname>%]{GRAY} @ {GREEN}CLIENT{C_RESET}    [%<message>%]"),
            error: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {RED}[%<levelname>%]{GRAY} @ {RED}CLIENT{C_RESET}   [%<message>%]"),
            ok: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {BLUE}INIT{C_RESET}{GRAY} @ {BLUE}CLIENT{C_RESET}    [%<message>%]"),
            warning: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {YELLOW}WARN{GRAY} @ {YELLOW}CLIENT{C_RESET}    [%<message>%]"),
            critical: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {RED}[%<levelname>%]{GRAY} @ {RED}CLIENT{C_RESET} [%<message>%]"),
            panic: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {RED}[%<levelname>%]{GRAY} @ {RED}CLIENT{C_RESET} [%<message>%]"),
            log_options: LogFormatOptions {
                timestamp_format: "%Y-%m-%d %H:%M:%S".to_string(),
                levelname_lowercase: false
            },
        }
    );
}
