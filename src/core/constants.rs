use lazy_static::lazy_static;

use libstrawberry::colors::{BLUE, BOLD, CYAN, C_RESET, GREEN, RED, YELLOW, GRAY};
use libstrawberry::logging::Logger;

pub const STRAWBERRY_ID_API: &str = "https://id.strawberryfoundations.org/v2/";
// pub const STRAWBERRY_ID_API: &str = "http://192.168.0.194:8082/v1/";

lazy_static! {
    pub static ref VERSION: String = env!("CARGO_PKG_VERSION").to_string();

    pub static ref LOGGER: Logger = Logger::new(
        libstrawberry::logging::features::LoggingFeatures::new(),
        libstrawberry::logging::formats::LogFormat {
            info: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {GREEN}[%<levelname>%]{C_RESET}    [%<message>%]"),
            error: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {RED}[%<levelname>%]{C_RESET}   [%<message>%]"),
            ok: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {BLUE}INIT{C_RESET}    [%<message>%]"),
            warning: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {YELLOW}[%<levelname>%]{C_RESET} [%<message>%]"),
            critical: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {RED}[%<levelname>%]{C_RESET} [%<message>%]"),
            panic: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {RED}[%<levelname>%]{C_RESET} [%<message>%]"),
            log_options: libstrawberry::logging::formats::LogFormatOptions {
                timestamp_format: "%Y-%m-%d %H:%M:%S".to_string(),
                levelname_lowercase: false
            },
        }
    );

    pub static ref LOGGER_2: Logger = Logger::new(
        libstrawberry::logging::features::LoggingFeatures::new(),
        libstrawberry::logging::formats::LogFormat {
            info: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {CYAN}AUTH{C_RESET}    [%<message>%]"),
            error: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {RED}[%<levelname>%]{C_RESET}   [%<message>%]"),
            ok: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {BLUE}INIT{C_RESET}    [%<message>%]"),
            warning: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {YELLOW}[%<levelname>%]{C_RESET} [%<message>%]"),
            critical: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {RED}[%<levelname>%]{C_RESET} [%<message>%]"),
            panic: format!("{C_RESET}{BOLD}{GRAY}[%<time>%]{C_RESET} {RED}[%<levelname>%]{C_RESET} [%<message>%]"),
            log_options: libstrawberry::logging::formats::LogFormatOptions {
                timestamp_format: "%Y-%m-%d %H:%M:%S".to_string(),
                levelname_lowercase: false
            },
        }
    );
}
