pub mod args;

use lazy_static::lazy_static;

lazy_static!(
    pub static ref ARGS: args::Args = args::Args::collect();
    pub static ref OPTIONS: args::Options = args::Args::collect().collect_options();
);