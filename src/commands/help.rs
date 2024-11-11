use stblib::colors::{C_RESET, GREEN, BOLD, UNDERLINE, CYAN, RESET, WHITE, RED, MAGENTA};
use crate::constants::VERSION;

pub fn help() {
    println!("\
{BOLD}{CYAN}{UNDERLINE}Strawberry Tunneled v{}{C_RESET}\n\
{GREEN}{BOLD}Usage:{RESET} {WHITE}tunneled {CYAN}[command] {RED}[<options>]{C_RESET}\n\n\
{MAGENTA}{BOLD}Commands:{C_RESET}
    {CYAN}{BOLD}help:{C_RESET} Prints this message
    {CYAN}{BOLD}about:{C_RESET} About Strawberry Tunneled
    {CYAN}{BOLD}auth:{C_RESET} Authenticate with your Strawberry ID

    {CYAN}{BOLD}local <port>:{C_RESET} Starts a local proxy to the remote server
     {BOLD}↳ {MAGENTA}Options:{C_RESET}
            {CYAN}{BOLD}-u, --use <server>{C_RESET}      Select your target server for tunneling your traffic
            {CYAN}{BOLD}-h, --address <host>{C_RESET}    The address to expose                 {GREEN}{BOLD}[default: localhost]{C_RESET}
            {CYAN}{BOLD}-p, --port <port>{C_RESET}       The port to expose                    {GREEN}{BOLD}[optional]{C_RESET}
            {CYAN}{BOLD}-s, --secret <secret>{C_RESET}   Secret for authentication             {GREEN}{BOLD}[optional]{C_RESET}
            {CYAN}{BOLD}-a, --auth{C_RESET}              Use Strawberry ID for Authentication  {GREEN}{BOLD}[optional]{C_RESET}
            {CYAN}{BOLD}-cp, --control-port{C_RESET}     Control port for remote proxy server  {GREEN}{BOLD}[default: 7835]{C_RESET}
            {CYAN}{BOLD}-sp, --static-port{C_RESET}      Static port forwarding (whitelist)    {GREEN}{BOLD}[optional]{C_RESET}

    {CYAN}{BOLD}server:{C_RESET} Runs the remote proxy server
     {BOLD}↳ {MAGENTA}Options:{C_RESET}
            {CYAN}{BOLD}-s, --secret <secret>{C_RESET}   Secret for authentication                 {GREEN}{BOLD}[optional]{C_RESET}
            {CYAN}{BOLD}-id, --require-id{C_RESET}       Enable Strawberry ID for Authentication   {GREEN}{BOLD}[optional]{C_RESET}
            {CYAN}{BOLD}-cp, --control-port{C_RESET}     Control port for proxy server             {GREEN}{BOLD}[default: 7835]{C_RESET}
            {CYAN}{BOLD}--min-port <port>{C_RESET}       Minimum Port for the remote proxy server  {GREEN}{BOLD}[default: 1024]{C_RESET}
            {CYAN}{BOLD}--max-port <port>{C_RESET}       Maximum Port for the remote proxy server  {GREEN}{BOLD}[default: 65535]{C_RESET}
            {CYAN}{BOLD}-f, --file <file>{C_RESET}       Configuration file for server config      {GREEN}{BOLD}[optional]{C_RESET}
            {CYAN}{BOLD}-v, --verbose    {C_RESET}       Enable verbose logging                    {GREEN}{BOLD}[optional]{C_RESET}

    {CYAN}{BOLD}compose:{C_RESET} Starts one or more local proxies for the remote server using a configuration file
     {BOLD}↳ {MAGENTA}Options:{C_RESET}
            {CYAN}{BOLD}-f, --file <file>{C_RESET}     Configuration file for proxy services   {GREEN}{BOLD}[default: service.yml]{C_RESET}
", *VERSION);
    std::process::exit(0);
}