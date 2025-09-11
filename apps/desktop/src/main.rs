use anyhow::Result;
use tracing::info;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

mod cli;
mod config;
mod logging;

use cli::Cli;
use config::ConfigManager;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    // Phase 1: Parse CLI arguments
    let args = Cli::parse_args();
    
    // Phase 2: Initialize logging
    logging::init_logging(args.log_level())?;
    info!("ZenTerm starting up...");
    
    // Phase 3: Load or create configuration
    let mut config_manager = ConfigManager::new()?;
    config_manager.load()?;
    info!("Config loaded from: {}", config_manager.config_path().display());
    
    // Phase 4: Handle reset flow if requested
    if args.reset_config {
        info!("Reset config flag detected");
        config_manager.reset()?;
    }
    
    // Phase 5: Run application (current winit loop or future TUI)
    run_application(&mut config_manager)?;
    
    // Phase 6: Persist configuration before exit
    config_manager.save()?;
    info!("ZenTerm shutting down gracefully");
    
    Ok(())
}

/// Run the main application logic
/// 
/// FUTURE_UI: This function will be replaced with TUI logic in the next PR.
/// Currently maintains existing winit behavior for compatibility.
fn run_application(config_manager: &mut ConfigManager) -> Result<()> {
    info!("Starting application with theme: {}", config_manager.config().general.theme);
    
    // For now, show engine status and maintain existing winit behavior
    println!("Engine says: {}", engine::hello());
    
    // Event loop e finestra base (existing behavior preserved)
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let window = WindowBuilder::new()
        .with_title("ZenTerm - Warp-like Terminal")
        .build(&event_loop)
        .expect("Failed to build window");

    info!("GUI window created, entering event loop");

    // Loop eventi (placeholder per renderer GPU e input handling)
    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                event,
                window_id: _,
            } => match event {
                WindowEvent::CloseRequested => {
                    info!("Window close requested");
                    elwt.exit();
                }
                WindowEvent::Resized(size) => {
                    // Qui in futuro: resize del renderer
                    info!("Window resized to: {}x{}", size.width, size.height);
                }
                WindowEvent::RedrawRequested => {
                    // Qui in futuro: render GPU (wgpu)
                }
                _ => {}
            },
            Event::AboutToWait => {
                // Qui in futuro: scheduling render/frame
                window.request_redraw();
            }
            _ => {}
        })
        .map_err(|e| anyhow::anyhow!("Event loop error: {}", e))?;

    Ok(())
}
