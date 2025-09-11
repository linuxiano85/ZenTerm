use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::io;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub first_run_completed: bool,
    pub theme: String,
    pub gpu_limit_percent: u8,
    pub telemetry_enabled: bool,
    pub gpu_acceleration: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            first_run_completed: false,
            theme: "dark".to_string(),
            gpu_limit_percent: 75,
            telemetry_enabled: false,
            gpu_acceleration: true,
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
    config: AppConfig,
}

impl ConfigManager {
    pub fn new() -> io::Result<Self> {
        let config_path = Self::get_config_path()?;
        let config = Self::load_or_default(&config_path)?;
        
        Ok(Self {
            config_path,
            config,
        })
    }

    fn get_config_path() -> io::Result<PathBuf> {
        let mut path = dirs::config_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
            .unwrap_or_else(|| PathBuf::from("."));
        
        path.push("zenterm");
        fs::create_dir_all(&path)?;
        path.push("config.toml");
        Ok(path)
    }

    fn load_or_default(config_path: &PathBuf) -> io::Result<AppConfig> {
        match fs::read_to_string(config_path) {
            Ok(content) => {
                match toml::from_str(&content) {
                    Ok(config) => {
                        info!("Loaded config from {:?}", config_path);
                        Ok(config)
                    }
                    Err(e) => {
                        warn!("Failed to parse config file, using defaults: {}", e);
                        Ok(AppConfig::default())
                    }
                }
            }
            Err(_) => {
                debug!("Config file not found, using defaults");
                Ok(AppConfig::default())
            }
        }
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut AppConfig {
        &mut self.config
    }

    pub fn save(&self) -> io::Result<()> {
        let content = toml::to_string_pretty(&self.config)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
        fs::write(&self.config_path, content)?;
        debug!("Saved config to {:?}", self.config_path);
        Ok(())
    }

    pub fn update_theme(&mut self, theme: String) {
        self.config.theme = theme;
        info!("settings.change.theme={}", self.config.theme);
    }

    pub fn update_gpu_limit(&mut self, limit: u8) {
        self.config.gpu_limit_percent = limit;
        info!("settings.change.gpu_limit={}", limit);
    }

    pub fn update_telemetry(&mut self, enabled: bool) {
        self.config.telemetry_enabled = enabled;
        info!("settings.change.telemetry={}", enabled);
    }

    pub fn update_gpu_acceleration(&mut self, enabled: bool) {
        self.config.gpu_acceleration = enabled;
        info!("settings.change.gpu_acceleration={}", enabled);
    }

    pub fn complete_first_run(&mut self) {
        self.config.first_run_completed = true;
        info!("wizard.complete");
    }
}