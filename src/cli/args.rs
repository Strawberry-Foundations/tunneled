use std::env;

pub enum Command {
    Local,
    Server,
    None
}

#[derive(Default)]
pub struct Options {
    pub min_port: u16,
    pub max_port: u16,
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

        args.args = parser.clone();
        args.command_str = parser.clone().first().unwrap().to_string();

        match args.command_str.as_str() {
            "local" => args.command = Command::Local,
            "server" => args.command = Command::Server,
            _ => args.command = Command::None,
        }

        args
    }

    pub fn collect_options(&mut self) -> Options {
        let mut options = Options {
            ..Default::default()
        };

        for (index, arg) in self.args.iter().enumerate() {
            match arg.as_str() {
                "--min-port" => options.min_port = self.args.get(index + 1).unwrap().parse().unwrap_or(1024).to_owned(),
                "--max-port" => options.max_port = self.args.get(index + 1).unwrap().parse().unwrap_or(65535).to_owned(),
                _ => {  }
            }
        }

        options
    }
}