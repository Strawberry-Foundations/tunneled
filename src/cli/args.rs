use std::env;
use stblib::colors::{BOLD, C_RESET, RED, RESET};
use std::collections::HashMap;

#[derive(Clone)]
pub enum Command {
    Local,
    Server,
    Compose,
    Auth,
    About,
    Plugin,
    None,
}

#[derive(Default)]
pub struct ServerOptions {
    pub min_port: u16,
    pub max_port: u16,
    pub secret: Option<String>,
    pub require_id: bool,
    pub control_port: u16,
    pub config_file: Option<String>,
}

#[derive(Default)]
pub struct ClientOptions {
    pub host: String,
    pub port: u16,
    pub server: String,
    pub secret: Option<String>,
    pub auth: bool,
    pub control_port: u16,
    pub static_port: Option<u16>,
    pub compose_file: Option<String>,
}

#[derive(Default)]
pub struct Options {
    pub server_options: ServerOptions,
    pub client_options: ClientOptions,
}

pub struct Args {
    pub args: Vec<String>,
    pub command: Command,
    pub command_str: String,
    pub options: Options,
}

impl Args {
    pub fn collect() -> Self {
        let args: Vec<String> = env::args().skip(1).collect();

        let mut result = Args {
            args: args.clone(),
            command: Command::None,
            command_str: args.first().cloned().unwrap_or_default(),
            options: Options::default(),
        };

        // Using a HashMap for command lookup for scalability
        let command_map = HashMap::from([
            ("local", Command::Local),
            ("server", Command::Server),
            ("auth", Command::Auth),
            ("about", Command::About),
            ("compose", Command::Compose),
            ("plugin", Command::Plugin),
        ]);

        result.command = command_map
            .get(result.command_str.as_str())
            .cloned()
            .unwrap_or(Command::None);

        result
    }

    pub fn collect_options(&mut self) -> Options {
        let mut options = Options {
            server_options: ServerOptions {
                min_port: 1024,
                max_port: 65535,
                control_port: 7835,
                ..Default::default()
            },
            client_options: ClientOptions {
                host: "localhost".to_string(),
                port: 8080,
                server: "strawberryfoundations.org".to_string(),
                control_port: 7835,
                ..Default::default()
            },
        };

        let mut iter = self.args.iter().skip(1);

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-p" | "--port" => parse_u16(iter.next(), &mut options.client_options.port, "port"),
                "-u" | "--use" => parse_string(iter.next(), &mut options.client_options.server, "server"),
                "-cp" | "--control-port" => parse_u16(
                    iter.next(),
                    match &self.command {
                        Command::Local => &mut options.client_options.control_port,
                        Command::Server => &mut options.server_options.control_port,
                        _ => continue,
                    },
                    "control port",
                ),
                "-sp" | "--static-port" => {
                    if let Some(port) = parse_optional_u16(iter.next(), "static port") {
                        options.client_options.static_port = Some(port);
                    }
                }
                "-h" | "--address" => parse_string(iter.next(), &mut options.client_options.host, "address"),
                "-f" | "--file" => parse_file(
                    iter.next(),
                    match &self.command {
                        Command::Compose => &mut options.client_options.compose_file,
                        Command::Server => &mut options.server_options.config_file,
                        _ => continue,
                    },
                    "file",
                ),
                "-s" | "--secret" => parse_optional_string(
                    iter.next(),
                    match &self.command {
                        Command::Local => &mut options.client_options.secret,
                        Command::Server => &mut options.server_options.secret,
                        _ => continue,
                    },
                    "secret",
                ),
                "--min-port" => parse_u16(iter.next(), &mut options.server_options.min_port, "minimum port"),
                "--max-port" => parse_u16(iter.next(), &mut options.server_options.max_port, "maximum port"),
                "-a" | "--auth" => options.client_options.auth = true,
                "-id" | "--require-id" => options.server_options.require_id = true,
                other => {
                    if let Ok(port) = other.parse::<u16>() {
                        options.client_options.port = port;
                    } else {
                        eprintln!("{RED}{BOLD} ! {RESET} Invalid port argument: {other}{C_RESET}");
                        std::process::exit(1);
                    }
                }
            }
        }

        options
    }
}

fn parse_u16(input: Option<&String>, field: &mut u16, field_name: &str) {
    if let Some(val) = input {
        *field = val.parse().unwrap_or_else(|_| {
            eprintln!("{RED}{BOLD} ! {RESET} Invalid {field_name}{C_RESET}");
            std::process::exit(1);
        });
    } else {
        eprintln!("{RED}{BOLD} ! {RESET} Missing {field_name}{C_RESET}");
    }
}

fn parse_string(input: Option<&String>, field: &mut String, field_name: &str) {
    if let Some(val) = input {
        *field = val.clone();
    } else {
        eprintln!("{RED}{BOLD} ! {RESET} Missing {field_name}{C_RESET}");
    }
}

fn parse_optional_string(input: Option<&String>, field: &mut Option<String>, field_name: &str) {
    if let Some(val) = input {
        *field = Some(val.clone());
    } else {
        eprintln!("{RED}{BOLD} ! {RESET} Missing {field_name}{C_RESET}");
    }
}


fn parse_optional_u16(input: Option<&String>, field_name: &str) -> Option<u16> {
    input.and_then(|val| {
        val.parse().ok().or_else(|| {
            eprintln!("{RED}{BOLD} ! {RESET} Invalid {field_name}{C_RESET}");
            None
        })
    })
}

fn parse_file(input: Option<&String>, field: &mut Option<String>, field_name: &str) {
    if let Some(val) = input {
        *field = Some(val.clone());
    } else {
        eprintln!("{RED}{BOLD} ! {RESET} Missing {field_name}{C_RESET}");
    }
}
