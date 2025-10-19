use clap::{Parser, Subcommand};
use eframe::egui;
use engine::{AppEvent, SharedAppState};
use log::{error, info};
use std::env;

#[derive(Parser)]
#[command(name = "zenterm")]
#[command(about = "ZenTerm Birthday MVP - Linux-first voice-driven terminal")]
#[command(version = "0.1.0-birthday-mvp")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Launch GUI mode
    #[arg(long)]
    gui: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Start TUI mode (placeholder)
    Tui,
}

fn main() -> Result<(), eframe::Error> {
    // Initialize logging
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let cli = Cli::parse();

    if cli.gui {
        info!("Starting ZenTerm GUI (Birthday MVP)");
        run_gui()
    } else if let Some(Commands::Tui) = cli.command {
        info!("TUI mode requested (placeholder implementation)");
        println!("TUI mode is not yet implemented. Use --gui to launch the GUI.");
        std::process::exit(1);
    } else {
        // Default to GUI if no command specified and running in graphical environment
        if is_graphical_environment() {
            info!("No command specified, defaulting to GUI mode");
            run_gui()
        } else {
            println!("ZenTerm Birthday MVP");
            println!("Usage: zenterm --gui  (launch GUI)");
            println!("       zenterm tui    (TUI mode - not implemented yet)");
            std::process::exit(1);
        }
    }
}

fn is_graphical_environment() -> bool {
    // Simple heuristic to detect if we're in a graphical environment
    env::var("DISPLAY").is_ok() || env::var("WAYLAND_DISPLAY").is_ok()
}

fn run_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("ZenTerm Birthday MVP")
            .with_resizable(true),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "ZenTerm",
        options,
        Box::new(|_cc| Box::new(ZenTermApp::new())),
    )
}

struct ZenTermApp {
    shared_state: SharedAppState,
    log_scroll_to_bottom: bool,
}

impl ZenTermApp {
    fn new() -> Self {
        let shared_state = SharedAppState::new();

        Self {
            shared_state,
            log_scroll_to_bottom: true,
        }
    }
}

impl eframe::App for ZenTermApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process events from the shared state
        self.shared_state.process_events();

        // Check if quit was requested
        if self.shared_state.is_quit_requested() {
            info!("Quit requested, exiting application");
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // Apply theme
        let theme = self.shared_state.get_theme();
        if theme.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        // Main layout
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Sidebar
                self.render_sidebar(ui);

                ui.separator();

                // Main content area
                ui.vertical(|ui| {
                    // Log panel (takes most of the space)
                    self.render_log_panel(ui);

                    ui.separator();

                    // Status bar
                    self.render_status_bar(ui);
                });
            });
        });

        // Wizard modal
        if self.shared_state.is_wizard_open() {
            self.render_wizard_modal(ctx);
        }

        // Request repaint for live updates (e.g., GPU usage, logs)
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}

impl ZenTermApp {
    fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.allocate_ui_with_layout(
            [200.0, ui.available_height()].into(),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                ui.heading("Controls");
                ui.separator();

                // GPU Limit buttons
                ui.label("GPU Limit:");
                let config = self.shared_state.get_config();
                let current_limit = config.gpu.limit_percentage;

                ui.horizontal(|ui| {
                    for limit in [25, 50, 75, 100] {
                        let selected = current_limit == limit;
                        if ui
                            .selectable_label(selected, format!("{}%", limit))
                            .clicked()
                            && !selected
                        {
                            let sender = self.shared_state.get_event_sender();
                            if let Err(e) = sender.send(AppEvent::GpuLimitChanged(limit)) {
                                error!("Failed to send GPU limit change event: {}", e);
                            }
                        }
                    }
                });

                ui.separator();

                // Theme toggle
                let theme = self.shared_state.get_theme();
                if ui.button(format!("Theme: {}", theme.name())).clicked() {
                    let sender = self.shared_state.get_event_sender();
                    if let Err(e) = sender.send(AppEvent::ThemeToggled(!theme.dark_mode)) {
                        error!("Failed to send theme toggle event: {}", e);
                    }
                }

                // Voice toggle
                let voice_status = self.shared_state.get_voice_status();
                if ui.button(format!("Voice: {}", voice_status)).clicked() {
                    let sender = self.shared_state.get_event_sender();
                    let new_state = voice_status == "OFF";
                    if let Err(e) = sender.send(AppEvent::VoiceToggled(new_state)) {
                        error!("Failed to send voice toggle event: {}", e);
                    }
                }

                ui.separator();

                // Wizard launcher
                if ui.button("Setup Wizard").clicked() {
                    let sender = self.shared_state.get_event_sender();
                    if let Err(e) = sender.send(AppEvent::WizardOpened) {
                        error!("Failed to send wizard open event: {}", e);
                    }
                }

                ui.separator();

                // Quit button
                if ui.button("Quit").clicked() {
                    let sender = self.shared_state.get_event_sender();
                    if let Err(e) = sender.send(AppEvent::QuitRequested) {
                        error!("Failed to send quit event: {}", e);
                    }
                }
            },
        );
    }

    fn render_log_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Live Log");

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(self.log_scroll_to_bottom)
            .show(ui, |ui| {
                let log_messages = self.shared_state.get_log_messages(100);

                if log_messages.is_empty() {
                    ui.colored_label(egui::Color32::GRAY, "No log messages yet...");
                } else {
                    for entry in &log_messages {
                        let color = match entry.level {
                            engine::shared_state::LogLevel::Error => egui::Color32::RED,
                            engine::shared_state::LogLevel::Warning => egui::Color32::YELLOW,
                            engine::shared_state::LogLevel::Info => ui.visuals().text_color(),
                            engine::shared_state::LogLevel::Debug => egui::Color32::GRAY,
                        };

                        ui.horizontal(|ui| {
                            ui.colored_label(
                                egui::Color32::GRAY,
                                format!("[{:.3}s]", entry.timestamp.elapsed().as_secs_f32()),
                            );
                            ui.colored_label(color, &entry.message);
                        });
                    }
                }
            });
    }

    fn render_status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // GPU status
            let (gpu_limit, gpu_usage) = self.shared_state.get_gpu_status();
            ui.label(format!("GPU: {} ({}% limit)", gpu_usage, gpu_limit));

            ui.separator();

            // Theme status
            let theme = self.shared_state.get_theme();
            ui.label(format!("Theme: {}", theme.name()));

            ui.separator();

            // Voice status
            let voice_status = self.shared_state.get_voice_status();
            ui.label(format!("Voice: {}", voice_status));

            ui.separator();

            // Config dirty indicator
            if self.shared_state.is_config_dirty() {
                ui.colored_label(egui::Color32::YELLOW, "●");
                ui.label("Config Modified");
            } else {
                ui.colored_label(egui::Color32::GREEN, "●");
                ui.label("Config Saved");
            }

            // Right-aligned build tag
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.colored_label(egui::Color32::LIGHT_BLUE, "Birthday MVP");
            });
        });
    }

    fn render_wizard_modal(&mut self, ctx: &egui::Context) {
        // For now, just show a simple modal - in a full implementation this would be multi-step
        egui::Window::new("Setup Wizard")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Welcome to ZenTerm!");
                    ui.separator();

                    ui.label("This is the Birthday MVP setup wizard.");
                    ui.label("Full multi-step wizard implementation coming soon.");

                    ui.separator();

                    if ui.button("Close").clicked() {
                        let sender = self.shared_state.get_event_sender();
                        if let Err(e) = sender.send(AppEvent::WizardClosed) {
                            error!("Failed to send wizard close event: {}", e);
                        }
                    }
                });
            });
    }
}
