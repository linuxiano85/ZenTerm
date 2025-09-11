use std::io;
use std::time::Instant;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use tracing::{debug, info, error};

use crate::config::ConfigManager;
use crate::gpu;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Wizard,
    Runtime,
    Settings,
}

pub struct TuiApp {
    mode: AppMode,
    wizard_step: usize,
    settings_cursor: usize,
    quit_requested: bool,
    dirty_flag: bool,
    last_change_instant: Option<Instant>,
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            mode: AppMode::Wizard,
            wizard_step: 0,
            settings_cursor: 0,
            quit_requested: false,
            dirty_flag: false,
            last_change_instant: None,
        }
    }

    pub fn is_quit_requested(&self) -> bool {
        self.quit_requested
    }

    pub fn handle_key_event(&mut self, key: KeyCode, cfg_mgr: &mut ConfigManager) {
        match self.mode {
            AppMode::Wizard => self.handle_wizard_key(key, cfg_mgr),
            AppMode::Runtime => self.handle_runtime_key(key, cfg_mgr),
            AppMode::Settings => self.handle_settings_key(key, cfg_mgr),
        }
    }

    fn handle_wizard_key(&mut self, key: KeyCode, cfg_mgr: &mut ConfigManager) {
        match key {
            KeyCode::Char('q') => {
                if !cfg_mgr.config().first_run_completed {
                    // TODO: Add confirmation dialog
                    self.quit_requested = true;
                } else {
                    self.quit_requested = true;
                }
            }
            KeyCode::Enter | KeyCode::Tab => {
                if self.wizard_step < 2 {
                    self.wizard_step += 1;
                } else {
                    cfg_mgr.complete_first_run();
                    self.mode = AppMode::Runtime;
                    self.mark_dirty();
                    info!("wizard.complete");
                }
            }
            KeyCode::Char('d') | KeyCode::Char('l') => {
                if self.wizard_step == 0 {
                    let new_theme = if cfg_mgr.config().theme == "dark" { "light" } else { "dark" };
                    cfg_mgr.update_theme(new_theme.to_string());
                    self.mark_dirty();
                }
            }
            KeyCode::Char('2') => {
                if self.wizard_step == 1 {
                    cfg_mgr.update_gpu_limit(25);
                    gpu::apply_limit(25);
                    self.mark_dirty();
                }
            }
            KeyCode::Char('5') => {
                if self.wizard_step == 1 {
                    cfg_mgr.update_gpu_limit(50);
                    gpu::apply_limit(50);
                    self.mark_dirty();
                }
            }
            KeyCode::Char('7') => {
                if self.wizard_step == 1 {
                    cfg_mgr.update_gpu_limit(75);
                    gpu::apply_limit(75);
                    self.mark_dirty();
                }
            }
            KeyCode::Char('1') => {
                if self.wizard_step == 1 {
                    cfg_mgr.update_gpu_limit(100);
                    gpu::apply_limit(100);
                    self.mark_dirty();
                }
            }
            KeyCode::Char('y') => {
                if self.wizard_step == 2 {
                    cfg_mgr.update_telemetry(true);
                    self.mark_dirty();
                }
            }
            KeyCode::Char('n') => {
                if self.wizard_step == 2 {
                    cfg_mgr.update_telemetry(false);
                    self.mark_dirty();
                }
            }
            _ => {}
        }
    }

    fn handle_runtime_key(&mut self, key: KeyCode, _cfg_mgr: &mut ConfigManager) {
        match key {
            KeyCode::Char('q') => {
                self.quit_requested = true;
            }
            KeyCode::F(2) | KeyCode::Char('s') => {
                self.mode = AppMode::Settings;
                self.settings_cursor = 0;
                info!("Entered settings mode");
            }
            _ => {}
        }
    }

    fn handle_settings_key(&mut self, key: KeyCode, cfg_mgr: &mut ConfigManager) {
        match key {
            KeyCode::Char('q') => {
                self.quit_requested = true;
            }
            KeyCode::Esc => {
                self.mode = AppMode::Runtime;
                info!("Exited settings mode");
            }
            KeyCode::Up => {
                if self.settings_cursor > 0 {
                    self.settings_cursor -= 1;
                }
            }
            KeyCode::Down => {
                if self.settings_cursor < 3 {
                    self.settings_cursor += 1;
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                match self.settings_cursor {
                    0 => {
                        // Toggle theme
                        let new_theme = if cfg_mgr.config().theme == "dark" { "light" } else { "dark" };
                        cfg_mgr.update_theme(new_theme.to_string());
                        self.mark_dirty();
                    }
                    1 => {
                        // Cycle GPU limit
                        let current = cfg_mgr.config().gpu_limit_percent;
                        let new_limit = match current {
                            25 => 50,
                            50 => 75,
                            75 => 100,
                            _ => 25,
                        };
                        cfg_mgr.update_gpu_limit(new_limit);
                        gpu::apply_limit(new_limit);
                        self.mark_dirty();
                    }
                    2 => {
                        // Toggle telemetry
                        let new_value = !cfg_mgr.config().telemetry_enabled;
                        cfg_mgr.update_telemetry(new_value);
                        self.mark_dirty();
                    }
                    3 => {
                        // Toggle GPU acceleration
                        let new_value = !cfg_mgr.config().gpu_acceleration;
                        cfg_mgr.update_gpu_acceleration(new_value);
                        self.mark_dirty();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn mark_dirty(&mut self) {
        self.dirty_flag = true;
        self.last_change_instant = Some(Instant::now());
    }

    pub fn should_save(&self) -> bool {
        self.dirty_flag && 
        self.last_change_instant.map_or(false, |instant| 
            instant.elapsed().as_millis() >= 500
        )
    }

    pub fn mark_saved(&mut self) {
        self.dirty_flag = false;
        self.last_change_instant = None;
    }

    pub fn force_save_if_dirty(&self) -> bool {
        self.dirty_flag
    }

    pub fn render(&self, frame: &mut Frame, cfg_mgr: &ConfigManager) {
        match self.mode {
            AppMode::Wizard => self.render_wizard(frame, cfg_mgr),
            AppMode::Runtime => self.render_runtime(frame, cfg_mgr),
            AppMode::Settings => self.render_settings(frame, cfg_mgr),
        }
    }

    fn render_wizard(&self, frame: &mut Frame, cfg_mgr: &ConfigManager) {
        let area = frame.area();
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        // Header
        let header = Paragraph::new("ZenTerm Setup Wizard")
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(header, chunks[0]);

        // Wizard content
        let content = match self.wizard_step {
            0 => self.render_wizard_theme(cfg_mgr),
            1 => self.render_wizard_gpu(cfg_mgr),
            2 => self.render_wizard_telemetry(cfg_mgr),
            _ => Text::from("Wizard complete!"),
        };

        let content_widget = Paragraph::new(content)
            .block(Block::default().borders(Borders::ALL).title("Setup"));
        frame.render_widget(content_widget, chunks[1]);

        // Footer
        let footer_text = match self.wizard_step {
            0 => "Theme: Press 'd' for dark, 'l' for light | Enter/Tab: Next | q: Quit",
            1 => "GPU Limit: Press 2(25%), 5(50%), 7(75%), 1(100%) | Enter/Tab: Next | q: Quit",
            2 => "Telemetry: Press 'y' for yes, 'n' for no | Enter/Tab: Finish | q: Quit",
            _ => "Enter/Tab: Continue | q: Quit",
        };
        
        let footer = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, chunks[2]);
    }

    fn render_wizard_theme<'a>(&self, cfg_mgr: &'a ConfigManager) -> Text<'a> {
        Text::from(vec![
            Line::from("Step 1 of 3: Choose Theme"),
            Line::from(""),
            Line::from(vec![
                Span::raw("Current theme: "),
                Span::styled(
                    &cfg_mgr.config().theme,
                    Style::default().fg(if cfg_mgr.config().theme == "dark" { Color::White } else { Color::Black })
                        .add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(""),
            Line::from("Press 'd' for dark theme or 'l' for light theme"),
        ])
    }

    fn render_wizard_gpu<'a>(&self, cfg_mgr: &'a ConfigManager) -> Text<'a> {
        Text::from(vec![
            Line::from("Step 2 of 3: Set GPU Limit"),
            Line::from(""),
            Line::from(vec![
                Span::raw("Current limit: "),
                Span::styled(
                    format!("{}%", cfg_mgr.config().gpu_limit_percent),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(""),
            Line::from("Choose GPU limit preset:"),
            Line::from("  2 → 25%"),
            Line::from("  5 → 50%"),
            Line::from("  7 → 75%"),
            Line::from("  1 → 100%"),
        ])
    }

    fn render_wizard_telemetry<'a>(&self, cfg_mgr: &'a ConfigManager) -> Text<'a> {
        Text::from(vec![
            Line::from("Step 3 of 3: Telemetry Settings"),
            Line::from(""),
            Line::from(vec![
                Span::raw("Telemetry: "),
                Span::styled(
                    if cfg_mgr.config().telemetry_enabled { "Enabled" } else { "Disabled" },
                    Style::default().fg(if cfg_mgr.config().telemetry_enabled { Color::Green } else { Color::Red })
                        .add_modifier(Modifier::BOLD)
                ),
            ]),
            Line::from(""),
            Line::from("Help improve ZenTerm by sharing anonymous usage data?"),
            Line::from("Press 'y' for yes or 'n' for no"),
        ])
    }

    fn render_runtime(&self, frame: &mut Frame, cfg_mgr: &ConfigManager) {
        let area = frame.area();
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        // Header with status
        let status_text = format!(
            "ZenTerm Runtime | Theme: {} | GPU: {}% | Telemetry: {}",
            cfg_mgr.config().theme,
            cfg_mgr.config().gpu_limit_percent,
            if cfg_mgr.config().telemetry_enabled { "On" } else { "Off" }
        );
        
        let header = Paragraph::new(status_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(header, chunks[0]);

        // Main area placeholder
        let main_content = Paragraph::new("Main terminal area placeholder\n\nThis will contain the terminal emulator in future versions.")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Terminal"));
        frame.render_widget(main_content, chunks[1]);

        // Footer
        let footer = Paragraph::new("F2/s: Settings | q: Quit")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, chunks[2]);
    }

    fn render_settings(&self, frame: &mut Frame, cfg_mgr: &ConfigManager) {
        let area = frame.area();
        
        // Center the settings panel
        let popup_area = centered_rect(60, 50, area);
        
        // Clear the background
        frame.render_widget(Clear, popup_area);
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(popup_area);

        // Header
        let header = Paragraph::new("Settings")
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(header, chunks[0]);

        // Settings list
        let settings_items = vec![
            ListItem::new(vec![
                Line::from(vec![
                    Span::raw("Theme: "),
                    Span::styled(
                        &cfg_mgr.config().theme,
                        Style::default().fg(Color::Green)
                    ),
                ])
            ]),
            ListItem::new(vec![
                Line::from(vec![
                    Span::raw("GPU Limit: "),
                    Span::styled(
                        format!("{}%", cfg_mgr.config().gpu_limit_percent),
                        Style::default().fg(Color::Green)
                    ),
                ])
            ]),
            ListItem::new(vec![
                Line::from(vec![
                    Span::raw("Telemetry: "),
                    Span::styled(
                        if cfg_mgr.config().telemetry_enabled { "Enabled" } else { "Disabled" },
                        Style::default().fg(if cfg_mgr.config().telemetry_enabled { Color::Green } else { Color::Red })
                    ),
                ])
            ]),
            ListItem::new(vec![
                Line::from(vec![
                    Span::raw("GPU Acceleration: "),
                    Span::styled(
                        if cfg_mgr.config().gpu_acceleration { "Enabled" } else { "Disabled" },
                        Style::default().fg(if cfg_mgr.config().gpu_acceleration { Color::Green } else { Color::Red })
                    ),
                ])
            ]),
        ];

        let mut list_state = ListState::default();
        list_state.select(Some(self.settings_cursor));

        let settings_list = List::new(settings_items)
            .block(Block::default().borders(Borders::ALL).title("Settings"))
            .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
            .highlight_symbol("► ");

        frame.render_stateful_widget(settings_list, chunks[1], &mut list_state);

        // Footer
        let footer = Paragraph::new("Up/Down: Navigate | Enter/Space: Edit | Esc: Back | q: Quit")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, chunks[2]);
    }
}

// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

pub fn run(mut cfg_mgr: ConfigManager) -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Install panic hook to restore terminal
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        original_hook(panic_info);
    }));

    let mut app = TuiApp::new();
    
    // Set initial mode based on first run status
    if cfg_mgr.config().first_run_completed {
        app.mode = AppMode::Runtime;
        info!("Starting in runtime mode");
    } else {
        app.mode = AppMode::Wizard;
        info!("wizard.start");
    }

    let result = run_app(&mut terminal, &mut app, &mut cfg_mgr);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Take back the original panic hook
    let _ = std::panic::take_hook();

    result
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut TuiApp,
    cfg_mgr: &mut ConfigManager,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| app.render(f, cfg_mgr))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key_event(key.code, cfg_mgr);
                }
            }
        }

        // Handle config saving
        if app.should_save() {
            if let Err(e) = cfg_mgr.save() {
                error!("Failed to save config: {}", e);
            } else {
                debug!("Config saved automatically");
                app.mark_saved();
            }
        }

        if app.is_quit_requested() {
            // Force save if dirty before quitting
            if app.force_save_if_dirty() {
                if let Err(e) = cfg_mgr.save() {
                    error!("Failed to save config on quit: {}", e);
                } else {
                    debug!("Config saved on quit");
                }
            }
            break;
        }
    }

    Ok(())
}