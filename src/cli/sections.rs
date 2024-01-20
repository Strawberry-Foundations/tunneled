use crate::colors::{C_RESET, GREEN, BOLD, UNDERLINE, CYAN, RESET, WHITE, RED, MAGENTA};
use crate::VERSION;

// {GREEN}{BOLD}[default]{C_RESET}

pub fn help_section() {
    println!("\
{BOLD}{CYAN}{UNDERLINE}Strawberry Tunneled v{VERSION}{C_RESET}\n\
{GREEN}{BOLD}Usage:{RESET} {WHITE}tunneled {CYAN}[command] {RED}[<options>]{C_RESET}\n\n\
{MAGENTA}{BOLD}Commands:{C_RESET}
    {CYAN}{BOLD}help:{C_RESET} Prints this message
    {CYAN}{BOLD}about:{C_RESET} About Strawberry Tunneled

    {CYAN}{BOLD}local:{C_RESET} Starts a local proxy to the remote server
     {BOLD}↳ {MAGENTA}Options:{C_RESET}
            {CYAN}{BOLD}-s, --server <server>{C_RESET}   Select your target server for tunneling your traffic

    {CYAN}{BOLD}auth:{C_RESET} Authenticate with your Strawberry ID
     {BOLD}↳ {MAGENTA}Options:{C_RESET}
            {CYAN}{BOLD}-???, --???{C_RESET}             ???

    {CYAN}{BOLD}server:{C_RESET} Runs the remote proxy server
     {BOLD}↳ {MAGENTA}Options:{C_RESET}
            {CYAN}{BOLD}--min-port <port>{C_RESET}       Minimum Port for the remote proxy server  {GREEN}{BOLD}[default: 1024]{C_RESET}
            {CYAN}{BOLD}--max-port <port>{C_RESET}       Maximum Port for the remote proxy server  {GREEN}{BOLD}[default: 65535]{C_RESET}

");
    std::process::exit(0);
}