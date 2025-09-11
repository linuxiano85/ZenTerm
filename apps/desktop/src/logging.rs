use anyhow::Result;
use tracing::Level;
use tracing_subscriber::{filter::EnvFilter, fmt, prelude::*};

/// Initialize logging with the specified level
pub fn init_logging(level: Level) -> Result<()> {
    // Prevent double initialization
    static LOGGING_INITIALIZED: std::sync::Once = std::sync::Once::new();
    
    LOGGING_INITIALIZED.call_once(|| {
        let filter = EnvFilter::builder()
            .with_default_directive(level.into())
            .from_env_lossy();

        tracing_subscriber::registry()
            .with(
                fmt::layer()
                    .with_target(false)
                    .with_thread_ids(false)
                    .with_thread_names(false)
                    .compact()
            )
            .with(filter)
            .init();
    });

    Ok(())
}