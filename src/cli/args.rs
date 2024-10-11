use std::env;
use stblib::colors::{BOLD, C_RESET, RED, RESET};

pub enum Command {
    Local,
    Server,
    Compose,
    Auth,
    About,
    Plugin,
    None
}

#[derive(Default)]
pub struct ServerOptions {
    pub min_port: u16,
    pub max_port: u16,
    pub secret: Option<String>,
    pub require_id: bool,
    pub control_port: u16,
    pub config_file: Option<String>
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
    pub compose_file: Option<String>
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
        let mut args = Self {
            args: vec![],
            command: Command::None,
            command_str: String::new(),
            options: Options { ..Default::default() }
        };

        let collector: Vec<String> = env::args().collect();

        if collector.len() <= 1 {
            return args
        }

        let parser: Vec<String> = env::args().skip(1).collect();

        args.args.clone_from(&parser);
        args.command_str = parser.clone().first().unwrap().to_string();

        match args.command_str.as_str() {
            "local" => args.command = Command::Local,
            "server" => args.command = Command::Server,
            "auth" => args.command = Command::Auth,
            "about" => args.command = Command::About,
            "compose" => args.command = Command::Compose,
            "plugin" => args.command = Command::Plugin,
            _ => args.command = Command::None,
        }

        args
    }

    pub fn collect_options(&mut self) -> Options {
        let mut options = Options {
            server_options: ServerOptions {
                min_port: 1024,
                max_port: 65535, 
                secret: None,
                require_id: false,
                control_port: 7835,
                config_file: None,
            },
            client_options: ClientOptions {
                host: String::from("localhost"),
                port: 8080,
                server: String::from("strawberryfoundations.org"),
                secret: None,
                auth: false,
                control_port: 7835,
                static_port: None,
                compose_file: None,
            }
        };


        let mut iter = self.args.clone().into_iter().skip(1);

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-p" | "--port" => {
                    if let Some(val) = iter.next() {
                        if let Ok(port) = val.parse::<u16>() {
                            options.client_options.port = port;
                        } else {
                            eprintln!("{RED}{BOLD} ! {RESET} Invalid port{C_RESET}");
                        }
                    } else {
                        eprintln!("{RED}{BOLD} ! {RESET} Missing port{C_RESET}");
                    }
                },

                "-u" | "--use" => {
                    if let Some(val) = iter.next() {
                        if let Ok(server) = val.parse::<String>() {
                            options.client_options.server = server;
                        } else {
                            eprintln!("{RED}{BOLD} ! {RESET} Invalid server{C_RESET}");
                        }
                    } else {
                        eprintln!("{RED}{BOLD} ! {RESET} Missing server{C_RESET}");
                    }
                },

                "-cp" | "--control-port" => {
                    if let Some(val) = iter.next() {
                        if let Ok(port) = val.parse::<u16>() {
                            match &self.command {
                                Command::Local => options.client_options.control_port = port,
                                Command::Server => options.server_options.control_port = port,
                                _ => { }
                            }


                        } else {
                            eprintln!("{RED}{BOLD} ! {RESET} Invalid control port{C_RESET}");
                        }
                    } else {
                        eprintln!("{RED}{BOLD} ! {RESET} Missing control port{C_RESET}");
                    }
                },

                "-sp" | "--static-port" => {
                    if let Some(val) = iter.next() {
                        if let Ok(port) = val.parse::<u16>() {
                            options.client_options.static_port = Some(port)
                        } else {
                            eprintln!("{RED}{BOLD} ! {RESET} Invalid static port{C_RESET}");
                            std::process::exit(1);
                        }
                    } else {
                        eprintln!("{RED}{BOLD} ! {RESET} Missing static port{C_RESET}");
                        std::process::exit(1);
                    }
                },

                "-h" | "--address" => {
                    if let Some(val) = iter.next() {
                        if let Ok(host) = val.parse::<String>() {
                            options.client_options.host = host;
                        } else {
                            eprintln!("{RED}{BOLD} ! {RESET} Invalid address{C_RESET}");
                        }
                    } else {
                        eprintln!("{RED}{BOLD} ! {RESET} Missing address{C_RESET}");
                    }
                },

                "-f" | "--file" => {
                    if let Some(val) = iter.next() {
                        if let Ok(file) = val.parse::<String>() {
                            match &self.command {
                                Command::Compose => options.client_options.compose_file = Option::from(file),
                                Command::Server => options.server_options.config_file = Option::from(file),
                                _ => { }
                            }
                        } else {
                            match &self.command {
                                Command::Compose => eprintln!("{RED}{BOLD} ! {RESET} Invalid compose file{C_RESET}"),
                                Command::Server => eprintln!("{RED}{BOLD} ! {RESET} Invalid config file{C_RESET}"),
                                _ => { }
                            }
                        }
                    } else {
                        match &self.command {
                            Command::Compose => eprintln!("{RED}{BOLD} ! {RESET} Missing compose file{C_RESET}"),
                            Command::Server => eprintln!("{RED}{BOLD} ! {RESET} Missing config file{C_RESET}"),
                            _ => { }
                        }
                    }
                },

                "-s" | "--secret" => {
                    if let Some(val) = iter.next() {
                        if let Ok(secret) = val.parse::<String>() {
                            match &self.command {
                                Command::Local => options.client_options.secret = Option::from(secret),
                                Command::Server => options.server_options.secret = Option::from(secret),
                                _ => { }
                            }
                        } else {
                            eprintln!("{RED}{BOLD} ! {RESET} Invalid secret{C_RESET}");
                        }
                    } else {
                        eprintln!("{RED}{BOLD} ! {RESET} Missing secret{C_RESET}");
                    }
                },
                "--min-port" => {
                    if let Some(val) = iter.next() {
                        if let Ok(min_port) = val.parse::<u16>() {
                            options.server_options.min_port = min_port;
                        } else {
                            eprintln!("{RED}{BOLD} ! {RESET} Invalid minimum port{C_RESET}");
                        }
                    } else {
                        eprintln!("{RED}{BOLD} ! {RESET} Missing minimum port{C_RESET}");
                    }
                },

                "--max-port" => {
                    if let Some(val) = iter.next() {
                        if let Ok(max_port) = val.parse::<u16>() {
                            options.server_options.max_port = max_port;
                        } else {
                            eprintln!("{RED}{BOLD} ! {RESET} Invalid maximum port{C_RESET}");
                        }
                    } else {
                        eprintln!("{RED}{BOLD} ! {RESET} Missing maximum port{C_RESET}");
                    }
                },

                "-a" | "--auth" => options.client_options.auth = true,
                "-id" | "--require-id" => options.server_options.require_id = true,

                _ => {
                    let port = arg.parse::<u16>().unwrap_or_else(|_| {
                        eprintln!("{RED}{BOLD} ! {RESET} Invalid port{C_RESET}");
                        std::process::exit(1);
                    });

                    options.client_options.port = port
                }
            }
        }

        options
    }
}