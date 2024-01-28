use lazy_static::lazy_static;

use stblib::logging::Logger;
use stblib::colors::{BLUE, BOLD, C_RESET, CYAN, GREEN, RED, YELLOW};

pub const VERSION: &str = "2.0.0";
// pub const STRAWBERRY_ID_API: &str = "http://192.168.0.194:8082/v1/";
pub const STRAWBERRY_ID_API: &str = "https://id.strawberryfoundations.xyz/v1/";

lazy_static! {
    pub static ref LOGGER: Logger = Logger::new(
        stblib::logging::featureset::FeatureSet::new(),
        stblib::logging::formats::LogFormat {
            info: format!("{C_RESET}{BOLD}[%<time>%]{C_RESET} {GREEN}[%<levelname>%]{C_RESET}    [%<message>%]"),
            error: format!("{C_RESET}{BOLD}[%<time>%]{C_RESET} {RED}[%<levelname>%]{C_RESET}   [%<message>%]"),
            default: format!("{C_RESET}{BOLD}[%<time>%]{C_RESET} {BLUE}INIT{C_RESET}    [%<message>%]"),
            warning: format!("{C_RESET}{BOLD}[%<time>%]{C_RESET} {YELLOW}[%<levelname>%]{C_RESET} [%<message>%]"),
            critical: format!("{C_RESET}{BOLD}[%<time>%]{C_RESET} {RED}[%<levelname>%]{C_RESET} [%<message>%]"),
            extensions: stblib::logging::formats::LogFormatExt {
                time_fmt: "%Y-%m-%d %H:%M".to_string(),
                levelname_lowercase: false
            },
        }
    );

    pub static ref LOGGER_2: Logger = Logger::new(
        stblib::logging::featureset::FeatureSet::new(),
        stblib::logging::formats::LogFormat {
            info: format!("{C_RESET}{BOLD}[%<time>%]{C_RESET} {CYAN}AUTH{C_RESET}    [%<message>%]"),
            error: format!("{C_RESET}{BOLD}[%<time>%]{C_RESET} {RED}[%<levelname>%]{C_RESET}   [%<message>%]"),
            default: format!("{C_RESET}{BOLD}[%<time>%]{C_RESET} {BLUE}INIT{C_RESET}    [%<message>%]"),
            warning: format!("{C_RESET}{BOLD}[%<time>%]{C_RESET} {YELLOW}[%<levelname>%]{C_RESET} [%<message>%]"),
            critical: format!("{C_RESET}{BOLD}[%<time>%]{C_RESET} {RED}[%<levelname>%]{C_RESET} [%<message>%]"),
            extensions: stblib::logging::formats::LogFormatExt {
                time_fmt: "%Y-%m-%d %H:%M".to_string(),
                levelname_lowercase: false
            },
        }
    );
}