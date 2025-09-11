use clap::Parser;

/// ZenTerm - A modern terminal application
#[derive(Parser, Debug)]
#[command(name = "zenterm")]
#[command(about = "A modern terminal application", long_about = None)]
pub struct Cli {
    /// Reset configuration to defaults
    #[arg(long)]
    pub reset_config: bool,

    /// Increase verbosity level (-v for debug, -vv for trace)
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    pub verbose: u8,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Get the log level based on verbosity
    pub fn log_level(&self) -> tracing::Level {
        match self.verbose {
            0 => tracing::Level::INFO,
            1 => tracing::Level::DEBUG,
            _ => tracing::Level::TRACE,
        }
    }
}