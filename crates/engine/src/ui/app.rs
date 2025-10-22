use crate::ui::palette::{Palette, Theme};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use log::{info, warn};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Wizard,
    Runtime,
    Settings,
    Help,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WizardStep {
    Welcome,
    GpuConfig,
    Complete,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub gpu_limit: u32,
    pub theme: String,
    pub debounce_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            gpu_limit: 80,
            theme: "dark".to_string(),
            debounce_ms: 500,
        }
    }
}

pub struct TuiApp {
    pub mode: AppMode,
    pub wizard_step: WizardStep,
    pub show_help: bool,
    pub confirm_quit_in_wizard: bool,
    pub config: Config,
    pub config_path: PathBuf,
    pub palette: Palette,
    pub debounce_ms: u64,
    pub last_change: Option<Instant>,
    pub wizard_buf: Vec<String>,
    pub settings_buf: Vec<String>,
    pub running: bool,
    pub terminal_size: Rect,
}

impl TuiApp {
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("zenterm");
        
        fs::create_dir_all(&config_dir)?;
        let config_path = config_dir.join("config.json");
        
        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Config::default()
        };

        let debounce_ms = std::env::var("ZENTERM_SAVE_DEBOUNCE_MS")
            .ok()
            .and_then(|s| s.parse().ok())
            .filter(|&ms| ms >= 50 && ms <= 10000)
            .unwrap_or(config.debounce_ms);

        let palette = match config.theme.as_str() {
            "light" => Palette::light(),
            _ => Palette::dark(),
        };

        // Determine initial mode based on config existence
        let mode = if config_path.exists() {
            AppMode::Runtime
        } else {
            AppMode::Wizard
        };

        if mode == AppMode::Wizard {
            info!("wizard.start");
        }

        Ok(Self {
            mode,
            wizard_step: WizardStep::Welcome,
            show_help: false,
            confirm_quit_in_wizard: false,
            config,
            config_path,
            palette,
            debounce_ms,
            last_change: None,
            wizard_buf: Vec::new(),
            settings_buf: Vec::new(),
            running: true,
            terminal_size: Rect::default(),
        })
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        while self.running {
            self.terminal_size = terminal.size()?;
            terminal.draw(|f| self.ui(f))?;
            
            if let Ok(true) = event::poll(Duration::from_millis(16)) {
                if let Event::Key(key) = event::read()? {
                    self.handle_input(key)?;
                }
            }

            // Handle debounced saving
            if let Some(last_change) = self.last_change {
                if last_change.elapsed() >= Duration::from_millis(self.debounce_ms) {
                    self.save_config()?;
                    self.last_change = None;
                }
            }
        }
        Ok(())
    }

    fn ui(&mut self, f: &mut Frame) {
        let size = f.size();
        
        // Responsive layout
        let show_footer = size.height >= 18;
        let compact_header = size.height < 12;
        
        let chunks = if show_footer {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(if compact_header { 1 } else { 3 }),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(size)
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(if compact_header { 1 } else { 3 }),
                    Constraint::Min(0),
                ])
                .split(size)
        };

        // Header
        self.render_header(f, chunks[0], compact_header);
        
        // Main content
        match self.mode {
            AppMode::Wizard => self.render_wizard(f, chunks[1]),
            AppMode::Runtime => self.render_runtime(f, chunks[1]),
            AppMode::Settings => self.render_settings(f, chunks[1]),
            AppMode::Help => unreachable!(), // Help is rendered as overlay
        }
        
        // Footer (if space allows)
        if show_footer {
            self.render_footer(f, chunks[chunks.len() - 1]);
        }
        
        // Help overlay
        if self.show_help {
            self.render_help_overlay(f, size);
        }
        
        // Quit confirmation in wizard
        if self.confirm_quit_in_wizard {
            self.render_quit_confirmation(f, size);
        }
    }

    fn render_header(&self, f: &mut Frame, area: Rect, compact: bool) {
        let title = match self.mode {
            AppMode::Wizard => format!("ZenTerm Setup - Step {:?}", self.wizard_step),
            AppMode::Runtime => "ZenTerm".to_string(),
            AppMode::Settings => "ZenTerm Settings".to_string(),
            AppMode::Help => "ZenTerm Help".to_string(),
        };

        let header_style = if compact {
            Style::default().fg(self.palette.text_primary)
        } else {
            Style::default()
                .fg(self.palette.text_primary)
                .add_modifier(Modifier::BOLD)
        };

        let border_style = if self.mode == AppMode::Wizard {
            Style::default().fg(self.palette.border_accent)
        } else {
            Style::default().fg(self.palette.border)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(title.clone())
            .style(border_style);

        let paragraph = Paragraph::new(if compact { "" } else { &title })
            .block(block)
            .style(header_style);

        f.render_widget(paragraph, area);
    }

    fn render_wizard(&self, f: &mut Frame, area: Rect) {
        let content = match self.wizard_step {
            WizardStep::Welcome => vec![
                Line::from("Welcome to ZenTerm!"),
                Line::from(""),
                Line::from("This wizard will help you configure your terminal."),
                Line::from(""),
                Line::from("Press ENTER to continue, 'q' to quit"),
            ],
            WizardStep::GpuConfig => vec![
                Line::from(format!("GPU Memory Limit: {}%", self.config.gpu_limit)),
                Line::from(""),
                Line::from("Use ↑/↓ to adjust (10-100%), ENTER to continue"),
                Line::from("'q' to quit, 'b' to go back"),
            ],
            WizardStep::Complete => vec![
                Line::from("Configuration complete!"),
                Line::from(""),
                Line::from("Your settings have been saved."),
                Line::from(""),
                Line::from("Press ENTER to start using ZenTerm"),
            ],
        };

        let paragraph = Paragraph::new(content)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Setup Wizard")
                .style(Style::default().fg(self.palette.border_accent)))
            .style(Style::default().fg(self.palette.text_primary))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn render_runtime(&self, f: &mut Frame, area: Rect) {
        let content = vec![
            Line::from("ZenTerm is running!"),
            Line::from(""),
            Line::from("Available commands:"),
            Line::from("  's' - Open Settings"),
            Line::from("  '?' - Show Help"),
            Line::from("  'q' - Quit"),
        ];

        let paragraph = Paragraph::new(content)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Runtime")
                .style(Style::default().fg(self.palette.border)))
            .style(Style::default().fg(self.palette.text_primary))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn render_settings(&self, f: &mut Frame, area: Rect) {
        let content = vec![
            Line::from(format!("GPU Memory Limit: {}%", self.config.gpu_limit)),
            Line::from(format!("Theme: {}", self.config.theme)),
            Line::from(format!("Save Debounce: {}ms", self.debounce_ms)),
            Line::from(""),
            Line::from("Controls:"),
            Line::from("  ↑/↓ - Adjust GPU limit"),
            Line::from("  't' - Toggle theme"),
            Line::from("  ESC/q - Back to Runtime"),
            Line::from("  Ctrl+C - Graceful exit"),
        ];

        let paragraph = Paragraph::new(content)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Settings")
                .style(Style::default().fg(self.palette.border)))
            .style(Style::default().fg(self.palette.text_primary))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn render_footer(&self, f: &mut Frame, area: Rect) {
        let mode_text = format!("Mode: {:?}", self.mode);
        let help_text = "Press '?' for help";
        
        let footer_text = format!("{} | {}", mode_text, help_text);
        
        let paragraph = Paragraph::new(footer_text)
            .style(Style::default()
                .fg(self.palette.text_secondary)
                .bg(self.palette.surface));

        f.render_widget(paragraph, area);
    }

    fn render_help_overlay(&self, f: &mut Frame, area: Rect) {
        let popup_area = self.centered_rect(60, 70, area);
        
        f.render_widget(Clear, popup_area);
        
        let help_items = vec![
            "Global Keybindings:",
            "",
            "  ? - Toggle this help",
            "  ESC - Close help/dialogs",
            "",
            "Wizard Mode:",
            "  ENTER - Next step",
            "  q - Quit (with confirmation)",
            "  b - Back (if available)",
            "  ↑/↓ - Adjust values",
            "",
            "Runtime Mode:",
            "  s - Open Settings",
            "  q - Quit application",
            "",
            "Settings Mode:",
            "  ↑/↓ - Adjust GPU limit",
            "  t - Toggle theme",
            "  Ctrl+C - Graceful exit",
            "  ESC/q - Back to Runtime",
            "",
            "Press ? or ESC to close this help",
        ];

        let help_text: Vec<Line> = help_items.iter().map(|&item| Line::from(item)).collect();
        
        let paragraph = Paragraph::new(help_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Help")
                .style(Style::default().fg(self.palette.border_accent)))
            .style(Style::default().fg(self.palette.text_primary))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, popup_area);
    }

    fn render_quit_confirmation(&self, f: &mut Frame, area: Rect) {
        let popup_area = self.centered_rect(40, 30, area);
        
        f.render_widget(Clear, popup_area);
        
        let content = vec![
            Line::from("Quit ZenTerm?"),
            Line::from(""),
            Line::from("Your configuration will not be saved."),
            Line::from(""),
            Line::from("y - Yes, quit"),
            Line::from("n - No, continue"),
        ];
        
        let paragraph = Paragraph::new(content)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Confirm Quit")
                .style(Style::default().fg(self.palette.error)))
            .style(Style::default().fg(self.palette.text_primary))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, popup_area);
    }

    fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }

    fn handle_input(&mut self, key: KeyEvent) -> Result<()> {
        // Global help toggle
        if key.code == KeyCode::Char('?') && !self.confirm_quit_in_wizard {
            self.show_help = !self.show_help;
            if self.show_help {
                info!("help.show");
            } else {
                info!("help.hide");
            }
            return Ok(());
        }

        // Close help/dialogs with ESC
        if key.code == KeyCode::Esc {
            if self.show_help {
                self.show_help = false;
                info!("help.hide");
            } else if self.confirm_quit_in_wizard {
                self.confirm_quit_in_wizard = false;
            }
            return Ok(());
        }

        // Handle quit confirmation in wizard
        if self.confirm_quit_in_wizard {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    warn!("wizard.abort");
                    self.running = false;
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    self.confirm_quit_in_wizard = false;
                }
                _ => {}
            }
            return Ok(());
        }

        // Don't process other inputs when help is shown
        if self.show_help {
            return Ok(());
        }

        // Mode-specific input handling
        match self.mode {
            AppMode::Wizard => self.handle_wizard_input(key)?,
            AppMode::Runtime => self.handle_runtime_input(key)?,
            AppMode::Settings => self.handle_settings_input(key)?,
            AppMode::Help => unreachable!(),
        }

        Ok(())
    }

    fn handle_wizard_input(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => {
                self.confirm_quit_in_wizard = true;
            }
            KeyCode::Enter => {
                match self.wizard_step {
                    WizardStep::Welcome => {
                        self.wizard_step = WizardStep::GpuConfig;
                    }
                    WizardStep::GpuConfig => {
                        self.wizard_step = WizardStep::Complete;
                    }
                    WizardStep::Complete => {
                        self.save_config()?;
                        self.mode = AppMode::Runtime;
                        info!("wizard.complete");
                    }
                }
            }
            KeyCode::Char('b') => {
                match self.wizard_step {
                    WizardStep::GpuConfig => self.wizard_step = WizardStep::Welcome,
                    WizardStep::Complete => self.wizard_step = WizardStep::GpuConfig,
                    _ => {}
                }
            }
            KeyCode::Up => {
                if self.wizard_step == WizardStep::GpuConfig {
                    self.config.gpu_limit = (self.config.gpu_limit + 5).min(100);
                    self.mark_changed();
                }
            }
            KeyCode::Down => {
                if self.wizard_step == WizardStep::GpuConfig {
                    self.config.gpu_limit = (self.config.gpu_limit.saturating_sub(5)).max(10);
                    self.mark_changed();
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_runtime_input(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => {
                self.running = false;
            }
            KeyCode::Char('s') => {
                self.mode = AppMode::Settings;
                info!("settings.open");
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_settings_input(&mut self, key: KeyEvent) -> Result<()> {
        // Ctrl+C graceful exit
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.running = false;
            return Ok(());
        }

        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.mode = AppMode::Runtime;
                info!("settings.close");
            }
            KeyCode::Up => {
                let old_limit = self.config.gpu_limit;
                self.config.gpu_limit = (self.config.gpu_limit + 5).min(100);
                if old_limit != self.config.gpu_limit {
                    info!("settings.change.gpu_limit: {} -> {}", old_limit, self.config.gpu_limit);
                    if self.config.gpu_limit >= 90 {
                        info!("gpu.limit.apply: High GPU limit set to {}%", self.config.gpu_limit);
                    }
                    self.mark_changed();
                }
            }
            KeyCode::Down => {
                let old_limit = self.config.gpu_limit;
                self.config.gpu_limit = (self.config.gpu_limit.saturating_sub(5)).max(10);
                if old_limit != self.config.gpu_limit {
                    info!("settings.change.gpu_limit: {} -> {}", old_limit, self.config.gpu_limit);
                    if self.config.gpu_limit >= 90 {
                        info!("gpu.limit.apply: High GPU limit set to {}%", self.config.gpu_limit);
                    }
                    self.mark_changed();
                }
            }
            KeyCode::Char('t') => {
                let old_theme = self.config.theme.clone();
                self.palette.toggle_theme();
                self.config.theme = match self.palette.theme {
                    Theme::Dark => "dark".to_string(),
                    Theme::Light => "light".to_string(),
                };
                info!("settings.change.theme: {} -> {}", old_theme, self.config.theme);
                self.mark_changed();
            }
            _ => {}
        }
        Ok(())
    }

    fn mark_changed(&mut self) {
        self.last_change = Some(Instant::now());
    }

    fn save_config(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.config)?;
        fs::write(&self.config_path, content)?;
        info!("Configuration saved to {:?}", self.config_path);
        Ok(())
    }
}