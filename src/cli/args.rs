use std::env;

#[derive(Default)]
pub struct Options {
    pub min_port: u16,
    pub max_port: u16,
}

#[derive(Default)]
pub struct Args {
    pub args: Vec<String>,
    pub command: String,
    pub options: Options,
}

impl Args {
    pub fn collect() -> Self {
        let mut args = Self {
            ..Default::default()
        };

        let collector: Vec<String> = env::args().collect();

        if collector.len() <= 1 {
            return args
        }

        let parser: Vec<String> = env::args().skip(1).collect();

        args.args = parser.clone();
        args.command = parser.clone().first().unwrap().to_string();

        args
    }
}