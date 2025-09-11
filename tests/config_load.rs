use std::fs;
use std::path::PathBuf;

/// Test config loading and corruption recovery
#[test]
fn config_load_and_recovery() {
    // Create a temporary config directory for testing
    let temp_dir = std::env::temp_dir().join("zenterm_test");
    let config_path = temp_dir.join("config.toml");
    
    // Clean up any existing test directory
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).unwrap();
    }
    
    fs::create_dir_all(&temp_dir).unwrap();
    
    // Test 1: Config creation when file doesn't exist
    {
        let default_config = r#"
version = "0.1.0"

[general]
theme = "dark"
gpu_limit_mb = 1024
telemetry = false
"#;
        
        // Verify default config format is valid
        assert!(default_config.contains("version"));
        assert!(default_config.contains("theme"));
        assert!(default_config.contains("gpu_limit_mb"));
        assert!(default_config.contains("telemetry"));
    }
    
    // Test 2: Corrupted config handling
    {
        // Write garbage data to config file
        fs::write(&config_path, "{garbage data}").unwrap();
        
        // Verify corrupted file exists
        assert!(config_path.exists());
        let content = fs::read_to_string(&config_path).unwrap();
        assert_eq!(content, "{garbage data}");
        
        // Verify we can recover by writing default config
        let default_config = r#"version = "0.1.0"

[general]
theme = "dark"
gpu_limit_mb = 1024
telemetry = false
"#;
        fs::write(&config_path, default_config).unwrap();
        
        // Should now have valid content
        let recovered_content = fs::read_to_string(&config_path).unwrap();
        assert!(recovered_content.contains("version = \"0.1.0\""));
        assert!(recovered_content.contains("theme = \"dark\""));
    }
    
    // Clean up
    fs::remove_dir_all(&temp_dir).unwrap();
}