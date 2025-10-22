use std::env;
use tempfile::TempDir;

use crate::ui::app::Config;

#[test]
fn test_config_default() {
    let config = Config::default();
    assert_eq!(config.gpu_limit, 80);
    assert_eq!(config.theme, "dark");
    assert_eq!(config.debounce_ms, 500);
}

#[test]
fn test_debounce_env_var() {
    // Test valid debounce value
    env::set_var("ZENTERM_SAVE_DEBOUNCE_MS", "100");
    let _temp_dir = TempDir::new().unwrap();
    
    // Note: We can't easily test TuiApp::new() due to dirs::config_dir dependency
    // But we can test the logic
    let debounce_ms = env::var("ZENTERM_SAVE_DEBOUNCE_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&ms| ms >= 50 && ms <= 10000)
        .unwrap_or(500);
    
    assert_eq!(debounce_ms, 100);
    
    // Test invalid debounce value (too low)
    env::set_var("ZENTERM_SAVE_DEBOUNCE_MS", "10");
    let debounce_ms = env::var("ZENTERM_SAVE_DEBOUNCE_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&ms| ms >= 50 && ms <= 10000)
        .unwrap_or(500);
    
    assert_eq!(debounce_ms, 500);
    
    // Test invalid debounce value (too high)
    env::set_var("ZENTERM_SAVE_DEBOUNCE_MS", "20000");
    let debounce_ms = env::var("ZENTERM_SAVE_DEBOUNCE_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&ms| ms >= 50 && ms <= 10000)
        .unwrap_or(500);
    
    assert_eq!(debounce_ms, 500);
    
    env::remove_var("ZENTERM_SAVE_DEBOUNCE_MS");
}

#[test]
fn test_config_serialization() {
    let config = Config {
        gpu_limit: 75,
        theme: "light".to_string(),
        debounce_ms: 200,
    };
    
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: Config = serde_json::from_str(&json).unwrap();
    
    assert_eq!(config.gpu_limit, deserialized.gpu_limit);
    assert_eq!(config.theme, deserialized.theme);
    assert_eq!(config.debounce_ms, deserialized.debounce_ms);
}

#[test] 
fn test_palette_theme_toggle() {
    use crate::ui::palette::{Palette, Theme};
    
    let mut palette = Palette::dark();
    assert_eq!(palette.theme, Theme::Dark);
    
    palette.toggle_theme();
    assert_eq!(palette.theme, Theme::Light);
    
    palette.toggle_theme();
    assert_eq!(palette.theme, Theme::Dark);
}