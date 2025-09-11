use tracing_subscriber;
use engine::{config::ConfigManager, ui};

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Initialize configuration
    let cfg_mgr = match ConfigManager::new() {
        Ok(mgr) => mgr,
        Err(e) => {
            eprintln!("Failed to initialize configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Launch TUI
    let exit_code = ui::init_and_run(cfg_mgr);
    std::process::exit(exit_code);
}
