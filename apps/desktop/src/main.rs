use engine::{install_panic_guard, run_tui};

fn main() {
    // Initialize logging
    env_logger::init();
    
    // Install panic guard before doing anything else
    install_panic_guard();
    
    // Test panic handling (remove this line after testing)
    // panic!("Testing panic handler - this should restore terminal properly");
    
    // Run the TUI application
    if let Err(e) = run_tui() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
