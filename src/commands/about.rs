use stblib::colors::{C_RESET, BOLD, UNDERLINE, CYAN};
use crate::statics::VERSION;

pub fn about() {
    println!("\
{BOLD}{CYAN}{UNDERLINE}Strawberry Tunneled v{}{C_RESET}\n\
tunneled is a simple CLI tool for making local tcp tunnels
", *VERSION);
    std::process::exit(0);
}