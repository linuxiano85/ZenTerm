//! TUI module for ZenTerm MVP
//! 
//! Provides interactive terminal user interface using ratatui + crossterm,
//! including first-run wizard, runtime screen, and settings panel.

mod app;

pub use app::run;

use tracing::{error, info};
use crate::config::ConfigManager;

/// Initialize and run the TUI application
/// 
/// Falls back gracefully if terminal initialization fails
pub fn init_and_run(cfg_mgr: ConfigManager) -> i32 {
    // Try to initialize TUI directly since crossterm doesn't have is_tty check
    // in the version we're using
    match run(cfg_mgr) {
        Ok(()) => {
            info!("TUI exited successfully");
            0
        }
        Err(e) => {
            error!("TUI failed: {}", e);
            // For MVP, exit with error rather than fallback
            // Future versions could implement CLI wizard fallback
            1
        }
    }
}

/// Fallback to CLI wizard if TUI initialization fails
#[allow(dead_code)]
fn fallback_to_cli(mut cfg_mgr: ConfigManager) -> i32 {
    if !cfg_mgr.config().first_run_completed {
        info!("Falling back to CLI wizard");
        
        // Simple CLI wizard implementation
        println!("Welcome to ZenTerm!");
        println!("TUI is not available, using CLI setup...");
        
        // Basic CLI setup
        println!("Using default settings: dark theme, 75% GPU limit, telemetry disabled");
        cfg_mgr.config_mut().theme = "dark".to_string();
        cfg_mgr.config_mut().gpu_limit_percent = 75;
        cfg_mgr.config_mut().telemetry_enabled = false;
        cfg_mgr.complete_first_run();
        
        if let Err(e) = cfg_mgr.save() {
            eprintln!("Warning: Failed to save config: {}", e);
        }
        
        println!("Setup complete! Restart ZenTerm to use the application.");
    } else {
        println!("ZenTerm is configured. TUI is not available in this environment.");
    }
    
    0
}