#![deny(warnings)]

use clap::{Parser, Subcommand};
use eframe::egui;
use engine::{AppEvent, SharedAppState};
use log::{error, info};
use serde_json::Value;
use std::env;
use std::thread;
// reqwest is optional at runtime; we use blocking client in a background thread
use reqwest::blocking::Client;

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
    // Minimal local chat input for Birthday MVP (echo responder)
    chat_input: String,
    // Help overlay flag
    show_help: bool,
    // GUI wizard state (local representation that sends events to engine)
    wizard_open: bool,
    wizard_step: usize,
    wizard_gpu_limit: u8,
    wizard_theme_dark: bool,
    wizard_voice_enabled: bool,
}

impl ZenTermApp {
    fn new() -> Self {
        let shared_state = SharedAppState::new();

        Self {
            shared_state,
            log_scroll_to_bottom: true,
            chat_input: String::new(),
            show_help: false,
            wizard_open: false,
            wizard_step: 0,
            wizard_gpu_limit: 25,
            wizard_theme_dark: true,
            wizard_voice_enabled: false,
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

        // Keyboard handling: toggle help with '?' (text event) and close help with ESC
        ctx.input(|i| {
            for ev in &i.events {
                match ev {
                    egui::Event::Text(t) => {
                        if t == "?" {
                            self.show_help = !self.show_help;
                            if self.show_help {
                                info!("help.show");
                            } else {
                                info!("help.hide");
                            }
                        }
                    }
                    egui::Event::Key {
                        key, pressed: true, ..
                    } => {
                        if *key == egui::Key::Escape && self.show_help {
                            self.show_help = false;
                            info!("help.hide");
                        }
                        // If wizard is open, ESC cancels it via sending WizardClosed
                        if *key == egui::Key::Escape && self.wizard_open {
                            let sender = self.shared_state.get_event_sender();
                            if let Err(e) = sender.send(AppEvent::WizardClosed) {
                                error!("Failed to send wizard close event (ESC): {}", e);
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

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

                    // Chat input area (simple echo bot)
                    ui.separator();
                    self.render_chat_input(ui);

                    ui.separator();

                    // Status bar: show only if there's enough vertical space
                    let available_h = ui.available_height();
                    // threshold in logical pixels; tweak if needed
                    if available_h >= 120.0 {
                        self.render_status_bar(ui);
                    }
                });
            });
        });

        // Wizard modal
        // sync local wizard_open with shared state
        self.wizard_open = self.shared_state.is_wizard_open();
        if self.wizard_open {
            self.render_wizard_modal(ctx);
        }

        // Help overlay
        if self.show_help {
            self.render_help_overlay(ctx);
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
                // Help button (GUI equivalent of '?')
                if ui.button("Help").clicked() {
                    self.show_help = !self.show_help;
                    if self.show_help {
                        info!("help.show");
                    } else {
                        info!("help.hide");
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

    /// Minimal chat input area that sends a user message and a simple echo reply
    fn render_chat_input(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Single-line text input bound to self.chat_input
            let input = ui.text_edit_singleline(&mut self.chat_input);

            // Send on button click or Enter
            let send_clicked = ui.button("Send").clicked();

            if send_clicked || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
            {
                let trimmed = self.chat_input.trim().to_string();
                if !trimmed.is_empty() {
                    let sender = self.shared_state.get_event_sender();

                    // Post user's message to the live log
                    if let Err(e) = sender.send(AppEvent::LogMessage(format!("You: {}", trimmed))) {
                        error!("Failed to send user chat message event: {}", e);
                    }

                    // Try to read OpenAI key from env
                    match env::var("ZENTERM_OPENAI_KEY") {
                        Ok(key) if !key.trim().is_empty() => {
                            // Spawn a background thread to call OpenAI so we don't block the UI
                            let key = key.clone();
                            let prompt = trimmed.clone();
                            let sender_clone = sender.clone();
                            thread::spawn(move || {
                                // Build blocking client
                                let client = Client::builder().build();
                                match client {
                                    Ok(client) => {
                                        let body = serde_json::json!({
                                            "model": "gpt-3.5-turbo",
                                            "messages": [{"role":"user","content": prompt}],
                                            "max_tokens": 250,
                                        });
                                        let resp = client
                                            .post("https://api.openai.com/v1/chat/completions")
                                            .bearer_auth(key)
                                            .json(&body)
                                            .send();
                                        match resp {
                                            Ok(r) => match r.json::<Value>() {
                                                Ok(json) => {
                                                    if let Some(choice) =
                                                        json.get("choices").and_then(|c| c.get(0))
                                                    {
                                                        if let Some(msg) = choice
                                                            .get("message")
                                                            .and_then(|m| m.get("content"))
                                                            .and_then(|c| c.as_str())
                                                        {
                                                            let reply =
                                                                format!("Bot: {}", msg.trim());
                                                            let _ = sender_clone
                                                                .send(AppEvent::LogMessage(reply));
                                                            return;
                                                        }
                                                    }
                                                    let _ =
                                                        sender_clone.send(AppEvent::LogMessage(
                                                            "Bot: (no reply)".to_string(),
                                                        ));
                                                }
                                                Err(e) => {
                                                    let _ = sender_clone.send(
                                                        AppEvent::LogMessage(format!(
                                                            "Bot: failed to parse response: {}",
                                                            e
                                                        )),
                                                    );
                                                }
                                            },
                                            Err(e) => {
                                                let _ = sender_clone.send(AppEvent::LogMessage(
                                                    format!("Bot: request failed: {}", e),
                                                ));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        let _ = sender_clone.send(AppEvent::LogMessage(format!(
                                            "Bot: failed to create HTTP client: {}",
                                            e
                                        )));
                                    }
                                }
                            });
                        }
                        _ => {
                            // Fallback to local echo responder
                            let reply = format!("Bot: Echo: {}", trimmed);
                            if let Err(e) = sender.send(AppEvent::LogMessage(reply)) {
                                error!("Failed to send bot reply event: {}", e);
                            }
                        }
                    }

                    // Clear input after sending
                    self.chat_input.clear();
                }
            }
        });
    }

    fn render_help_overlay(&mut self, ctx: &egui::Context) {
        egui::Window::new("Help")
            .collapsible(false)
            .resizable(true)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .title_bar(true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("ZenTerm Help");
                    ui.separator();
                    ui.label("Global Keybindings:");
                    ui.label("  - Help (this window)");
                    ui.label("  - Setup Wizard from sidebar");
                    ui.label("  - Use the Live Log and sidebar controls to change GPU/Theme/Voice");
                    ui.separator();
                    ui.label("Wizard steps:");
                    ui.label("  1) GPU limit selection (25/50/75/100)");
                    ui.label("  2) Theme selection (dark/light)");
                    ui.label("  3) Voice setup (enable/disable)");
                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.show_help = false;
                        info!("help.hide");
                    }
                });
            });
    }

    fn render_wizard_modal(&mut self, ctx: &egui::Context) {
        // Simple multi-step wizard in GUI that forwards selection via events
        egui::Window::new("Setup Wizard")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    let titles = [
                        "Welcome",
                        "GPU Configuration",
                        "Theme Selection",
                        "Voice Setup",
                        "Complete",
                    ];

                    let descriptions = [
                        "Welcome to ZenTerm Birthday MVP! This wizard will help configure the app.",
                        "Select a GPU limit to help manage resources.",
                        "Pick a theme: dark or light.",
                        "Enable or disable the voice mock engine.",
                        "Setup complete. Save settings to finish.",
                    ];

                    let step = self.wizard_step.min(titles.len() - 1);
                    ui.heading(titles[step]);
                    ui.label(descriptions[step]);
                    ui.separator();

                    match step {
                        1 => {
                            ui.label("GPU Limit:");
                            ui.horizontal(|ui| {
                                for &limit in &[25u8, 50u8, 75u8, 100u8] {
                                    let selected = self.wizard_gpu_limit == limit;
                                    if ui
                                        .selectable_label(selected, format!("{}%", limit))
                                        .clicked()
                                    {
                                        self.wizard_gpu_limit = limit;
                                    }
                                }
                            });
                        }
                        2 => {
                            ui.horizontal(|ui| {
                                if ui.button("Dark").clicked() {
                                    self.wizard_theme_dark = true;
                                }
                                if ui.button("Light").clicked() {
                                    self.wizard_theme_dark = false;
                                }
                            });
                        }
                        3 => {
                            ui.horizontal(|ui| {
                                if ui
                                    .checkbox(&mut self.wizard_voice_enabled, "Enable Voice Mock")
                                    .clicked()
                                {
                                    // toggled via checkbox
                                }
                            });
                        }
                        _ => {}
                    }

                    ui.separator();
                    ui.horizontal(|ui| {
                        if self.wizard_step > 0 && ui.button("Back").clicked() {
                            self.wizard_step = self.wizard_step.saturating_sub(1);
                        }

                        if self.wizard_step + 1 < titles.len() {
                            if ui.button("Next").clicked() {
                                // Apply intermediate settings as events
                                match self.wizard_step {
                                    1 => {
                                        let sender = self.shared_state.get_event_sender();
                                        if let Err(e) = sender
                                            .send(AppEvent::GpuLimitChanged(self.wizard_gpu_limit))
                                        {
                                            error!("Failed to send GPU limit from wizard: {}", e);
                                        }
                                    }
                                    2 => {
                                        let sender = self.shared_state.get_event_sender();
                                        if let Err(e) = sender
                                            .send(AppEvent::ThemeToggled(self.wizard_theme_dark))
                                        {
                                            error!(
                                                "Failed to send theme toggle from wizard: {}",
                                                e
                                            );
                                        }
                                    }
                                    3 => {
                                        let sender = self.shared_state.get_event_sender();
                                        if let Err(e) = sender
                                            .send(AppEvent::VoiceToggled(self.wizard_voice_enabled))
                                        {
                                            error!(
                                                "Failed to send voice toggle from wizard: {}",
                                                e
                                            );
                                        }
                                    }
                                    _ => {}
                                }
                                self.wizard_step += 1;
                            }
                        } else {
                            // Finish
                            if ui.button("Finish").clicked() {
                                // send a save request
                                let sender = self.shared_state.get_event_sender();
                                if let Err(e) = sender.send(AppEvent::ConfigSaveRequested) {
                                    error!("Failed to request config save: {}", e);
                                }
                                // close wizard in shared state
                                if let Err(e) = sender.send(AppEvent::WizardClosed) {
                                    error!("Failed to send wizard close event: {}", e);
                                }
                                self.wizard_step = titles.len() - 1;
                            }
                        }
                        ui.add_space(8.0);
                        if ui.button("Cancel").clicked() {
                            let sender = self.shared_state.get_event_sender();
                            if let Err(e) = sender.send(AppEvent::WizardClosed) {
                                error!("Failed to send wizard close event: {}", e);
                            }
                        }
                    });
                });
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
}
