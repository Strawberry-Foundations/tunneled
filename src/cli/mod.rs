pub mod args;

use std::sync::LazyLock;

pub static ARGS: LazyLock<args::Args> = LazyLock::new(args::Args::collect);
pub static OPTIONS: LazyLock<args::Options> = LazyLock::new(|| {
    let mut args = args::Args::collect();
    args.collect_options()
});