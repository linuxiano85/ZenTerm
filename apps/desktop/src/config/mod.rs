use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tracing::{debug, info, warn};

/// Configuration structure for ZenTerm
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Application version (for migration purposes)
    pub version: String,
    
    /// General settings
    pub general: GeneralConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    /// Theme name
    pub theme: String,
    
    /// GPU memory limit in MB
    pub gpu_limit_mb: u32,
    
    /// Enable telemetry
    pub telemetry: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            general: GeneralConfig {
                theme: "dark".to_string(),
                gpu_limit_mb: 1024,
                telemetry: false,
            },
        }
    }
}

/// Configuration manager handles loading, saving, and resetting configuration
pub struct ConfigManager {
    config_path: PathBuf,
    config: Config,
    dirty: bool,
}

impl ConfigManager {
    /// Create a new ConfigManager with the default config directory
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .context("Failed to get config directory")?
            .join("zenterm");
        
        // Ensure config directory exists
        fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;
            
        let config_path = config_dir.join("config.toml");
        
        Ok(Self {
            config_path,
            config: Config::default(),
            dirty: false,
        })
    }
    
    /// Load configuration from file, creating default if not exists or corrupted
    pub fn load(&mut self) -> Result<()> {
        debug!("Loading config from: {}", self.config_path.display());
        
        if !self.config_path.exists() {
            info!("Config file not found, creating default configuration");
            self.config = Config::default();
            self.dirty = true;
            self.save()?;
            return Ok(());
        }

        match fs::read_to_string(&self.config_path) {
            Ok(content) => {
                match toml::from_str::<Config>(&content) {
                    Ok(config) => {
                        info!("Successfully loaded configuration");
                        self.config = config;
                        self.dirty = false;
                    }
                    Err(e) => {
                        warn!("Failed to parse config file: {}. Using default configuration.", e);
                        self.config = Config::default();
                        self.dirty = true;
                        self.save()?;
                    }
                }
            }
            Err(e) => {
                warn!("Failed to read config file: {}. Using default configuration.", e);
                self.config = Config::default();
                self.dirty = true;
                self.save()?;
            }
        }
        
        Ok(())
    }
    
    /// Reset configuration to defaults
    pub fn reset(&mut self) -> Result<()> {
        info!("Resetting configuration to defaults");
        self.config = Config::default();
        self.dirty = true;
        self.save()?;
        Ok(())
    }
    
    /// Save configuration to file atomically
    pub fn save(&mut self) -> Result<()> {
        if !self.dirty {
            debug!("Configuration not dirty, skipping save");
            return Ok(());
        }
        
        debug!("Saving config to: {}", self.config_path.display());
        
        let content = toml::to_string_pretty(&self.config)
            .context("Failed to serialize configuration")?;
            
        // Atomic save: write to temp file, then rename
        let temp_path = self.config_path.with_extension("config.toml.tmp");
        
        fs::write(&temp_path, &content)
            .context("Failed to write temporary config file")?;
            
        fs::rename(&temp_path, &self.config_path)
            .context("Failed to rename temporary config file")?;
            
        self.dirty = false;
        info!("Configuration saved successfully");
        Ok(())
    }
    
    /// Get immutable reference to config
    pub fn config(&self) -> &Config {
        &self.config
    }
    
    /// Get the config file path
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }
}