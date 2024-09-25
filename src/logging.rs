use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

use crate::Verbosity;

pub fn initialize(verbosity: &Verbosity) -> anyhow::Result<()> {
    TermLogger::init(
        match verbosity {
            Verbosity::Debug => LevelFilter::Debug,
            Verbosity::Info => LevelFilter::Info,
            Verbosity::Warn => LevelFilter::Warn,
            Verbosity::Error => LevelFilter::Error,
        },
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )
    .map_err(|e| e.into())
}
