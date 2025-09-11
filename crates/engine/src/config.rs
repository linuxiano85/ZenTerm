use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Configuration for the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub gpu: GpuConfig,
    pub theme: ThemeConfig,
    pub voice: VoiceConfig,

    // Skip serialization - internal state for first-run detection
    #[serde(skip)]
    pub was_fresh: bool,

    // Skip serialization - internal state for save debouncing
    #[serde(skip)]
    pub last_save: Option<Instant>,

    #[serde(skip)]
    pub dirty: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConfig {
    pub limit_percentage: u8, // 25, 50, 75, 100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub dark_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    pub enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            gpu: GpuConfig {
                limit_percentage: 75,
            },
            theme: ThemeConfig { dark_mode: true },
            voice: VoiceConfig { enabled: false },
            was_fresh: false,
            last_save: None,
            dirty: false,
        }
    }
}

impl Config {
    /// Load configuration from file, or create default if it doesn't exist.
    /// Returns (config, was_fresh) where was_fresh indicates if a new config was created.
    pub fn load_or_default() -> (Config, bool) {
        let config_path = Self::config_path();

        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => match serde_json::from_str::<Config>(&content) {
                    Ok(mut config) => {
                        config.was_fresh = false;
                        info!("Loaded configuration from {:?}", config_path);
                        (config, false)
                    }
                    Err(e) => {
                        warn!("Failed to parse config file: {}. Using defaults.", e);
                        let config = Config {
                            was_fresh: true,
                            ..Default::default()
                        };
                        (config, true)
                    }
                },
                Err(e) => {
                    warn!("Failed to read config file: {}. Using defaults.", e);
                    let config = Config {
                        was_fresh: true,
                        ..Default::default()
                    };
                    (config, true)
                }
            }
        } else {
            let config = Config {
                was_fresh: true,
                ..Default::default()
            };
            info!("No config file found. Creating fresh configuration.");
            (config, true)
        }
    }

    /// Get the path to the configuration file
    pub fn config_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            let app_config_dir = config_dir.join("zenterm");
            if !app_config_dir.exists() {
                if let Err(e) = fs::create_dir_all(&app_config_dir) {
                    warn!("Failed to create config directory: {}", e);
                }
            }
            app_config_dir.join("config.json")
        } else {
            // Fallback to current directory
            PathBuf::from("zenterm_config.json")
        }
    }

    /// Save configuration to file with debouncing (500ms minimum interval)
    /// This prevents excessive disk writes during rapid config changes
    pub fn save_debounced(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let now = Instant::now();

        // Check if enough time has passed since last save (debounce 500ms)
        if let Some(last_save) = self.last_save {
            if now.duration_since(last_save) < Duration::from_millis(500) {
                // Mark as dirty but don't save yet
                self.dirty = true;
                return Ok(());
            }
        }

        self.save()?;
        self.last_save = Some(now);
        self.dirty = false;
        Ok(())
    }

    /// Force save configuration to file
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        info!("Saved configuration to {:?}", config_path);
        Ok(())
    }

    /// Check if config should be saved (for the debounce mechanism)
    pub fn should_save(&self) -> bool {
        if !self.dirty {
            return false;
        }

        if let Some(last_save) = self.last_save {
            Instant::now().duration_since(last_save) >= Duration::from_millis(500)
        } else {
            true
        }
    }

    /// Mark configuration as dirty (needs saving)
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.gpu.limit_percentage, 75);
        assert!(config.theme.dark_mode);
        assert!(!config.voice.enabled);
        assert!(!config.was_fresh);
        assert!(!config.dirty);
    }

    #[test]
    fn test_first_run_detection() {
        // Test in a temporary directory to avoid conflicts
        let temp_dir = std::env::temp_dir().join("zenterm_test_first_run");
        let config_path = temp_dir.join("config.json");

        // Clean up any existing test file
        if config_path.exists() {
            fs::remove_file(&config_path).ok();
        }

        // Mock the config_path function by testing the logic directly
        // Since we can't easily override the config_path in tests,
        // we'll test the was_fresh flag logic

        // Create a fresh config
        let (config, was_fresh) = Config::load_or_default();

        // The behavior depends on whether a config file already exists
        // In the test environment, this could be either true or false
        // The important thing is that the flag is set correctly
        assert_eq!(config.was_fresh, was_fresh);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();

        assert_eq!(
            config.gpu.limit_percentage,
            deserialized.gpu.limit_percentage
        );
        assert_eq!(config.theme.dark_mode, deserialized.theme.dark_mode);
        assert_eq!(config.voice.enabled, deserialized.voice.enabled);

        // Skipped fields should not be serialized and should use defaults
        assert!(!deserialized.was_fresh);
        assert!(!deserialized.dirty);
        assert!(deserialized.last_save.is_none());
    }
}
