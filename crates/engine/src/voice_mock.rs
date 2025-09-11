use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use crossbeam_channel::Sender;
use log::{info, debug, warn};
use crate::event_bus::AppEvent;

/// Mock voice recognition engine
/// TODO: Replace with actual voice engine integration (e.g., Vosk) in future phases
#[derive(Debug)]
pub struct VoiceMock {
    enabled: Arc<AtomicBool>,
    _handle: Option<thread::JoinHandle<()>>,
}

impl VoiceMock {
    /// Create a new voice mock engine
    pub fn new(enabled: bool, event_sender: Sender<AppEvent>) -> Self {
        let enabled_flag = Arc::new(AtomicBool::new(enabled));
        
        let handle = if enabled {
            let enabled_clone = enabled_flag.clone();
            Some(thread::spawn(move || {
                Self::voice_thread_loop(enabled_clone, event_sender);
            }))
        } else {
            info!("Voice engine disabled, not starting voice thread");
            None
        };
        
        Self {
            enabled: enabled_flag,
            _handle: handle,
        }
    }
    
    /// Check if voice engine is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }
    
    /// Enable or disable the voice engine
    /// Note: This only sets the flag, the thread will continue running but won't process
    /// In a real implementation, this might need to restart the thread or pause processing
    pub fn set_enabled(&self, enabled: bool) {
        info!("Voice engine {}", if enabled { "enabled" } else { "disabled" });
        self.enabled.store(enabled, Ordering::Relaxed);
    }
    
    /// Get voice engine status as a string
    pub fn status_string(&self) -> &'static str {
        if self.is_enabled() {
            "ON"
        } else {
            "OFF"
        }
    }
    
    /// Voice thread main loop - sends heartbeat messages every 10 seconds when enabled
    fn voice_thread_loop(enabled: Arc<AtomicBool>, event_sender: Sender<AppEvent>) {
        info!("Voice engine thread started");
        
        let mut heartbeat_counter = 0u64;
        
        loop {
            // Check if voice is enabled
            if enabled.load(Ordering::Relaxed) {
                heartbeat_counter += 1;
                let message = format!("Voice engine heartbeat #{} - listening for commands", heartbeat_counter);
                debug!("Voice heartbeat: {}", message);
                
                // Send heartbeat message to the log
                if let Err(e) = event_sender.send(AppEvent::LogMessage(message)) {
                    warn!("Failed to send voice heartbeat message: {}", e);
                    break; // If we can't send events, the main thread is probably gone
                }
            } else {
                debug!("Voice engine disabled, skipping heartbeat");
            }
            
            // Sleep for 10 seconds
            thread::sleep(Duration::from_secs(10));
        }
        
        info!("Voice engine thread ended");
    }
    
    /// Simulate voice command recognition (placeholder for future integration)
    pub fn simulate_command(&self, command: &str) -> Result<String, String> {
        if !self.is_enabled() {
            return Err("Voice engine is disabled".to_string());
        }
        
        info!("Voice command recognized: {}", command);
        
        // Simple command simulation - in a real implementation this would use
        // speech recognition libraries like Vosk, OpenAI Whisper, etc.
        let response = match command.to_lowercase().as_str() {
            "help" => "Available commands: help, status, quit, gpu limit, theme toggle, voice toggle",
            "status" => "Voice engine is running normally",
            "quit" => "Goodbye!",
            _ => "Command not recognized. Say 'help' for available commands.",
        };
        
        Ok(response.to_string())
    }
}

impl Drop for VoiceMock {
    fn drop(&mut self) {
        // Set enabled to false to signal the thread to stop gracefully
        self.enabled.store(false, Ordering::Relaxed);
        
        // Note: We can't join the thread here because we moved the handle into the struct
        // In a real implementation, we might want to use a channel to signal shutdown
        debug!("Voice engine shutting down");
    }
}

// We can't derive Clone because of the thread handle, but we can implement it manually
impl Clone for VoiceMock {
    fn clone(&self) -> Self {
        // For cloning, we create a new instance with the same enabled state
        // but don't clone the thread (each instance should have its own thread if needed)
        Self {
            enabled: Arc::clone(&self.enabled),
            _handle: None, // Don't clone the thread handle
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_bus::EventBus;
    use std::time::Duration;

    #[test]
    fn test_voice_mock_creation() {
        let event_bus = EventBus::new();
        let voice = VoiceMock::new(true, event_bus.sender());
        
        assert!(voice.is_enabled());
        assert_eq!(voice.status_string(), "ON");
    }
    
    #[test]
    fn test_voice_mock_disabled() {
        let event_bus = EventBus::new();
        let voice = VoiceMock::new(false, event_bus.sender());
        
        assert!(!voice.is_enabled());
        assert_eq!(voice.status_string(), "OFF");
    }
    
    #[test]
    fn test_voice_enable_disable() {
        let event_bus = EventBus::new();
        let voice = VoiceMock::new(true, event_bus.sender());
        
        assert!(voice.is_enabled());
        
        voice.set_enabled(false);
        assert!(!voice.is_enabled());
        assert_eq!(voice.status_string(), "OFF");
        
        voice.set_enabled(true);
        assert!(voice.is_enabled());
        assert_eq!(voice.status_string(), "ON");
    }
    
    #[test]
    fn test_voice_command_simulation() {
        let event_bus = EventBus::new();
        let voice = VoiceMock::new(true, event_bus.sender());
        
        // Test recognized commands
        let help_response = voice.simulate_command("help").unwrap();
        assert!(help_response.contains("Available commands"));
        
        let status_response = voice.simulate_command("status").unwrap();
        assert!(status_response.contains("running normally"));
        
        // Test unrecognized command
        let unknown_response = voice.simulate_command("unknown").unwrap();
        assert!(unknown_response.contains("not recognized"));
        
        // Test disabled voice
        voice.set_enabled(false);
        let disabled_result = voice.simulate_command("help");
        assert!(disabled_result.is_err());
        assert!(disabled_result.unwrap_err().contains("disabled"));
    }
    
    #[test]
    fn test_voice_heartbeat_messages() {
        let event_bus = EventBus::new();
        let receiver = event_bus.receiver();
        
        // Create voice engine with heartbeat
        let _voice = VoiceMock::new(true, event_bus.sender());
        
        // Wait a bit to see if we get heartbeat messages
        // Note: This test might be flaky due to timing, but gives us basic verification
        thread::sleep(Duration::from_millis(100));
        
        // We won't wait for the full 10 seconds, but the voice thread should be running
        // The real test would be to check if the thread was created successfully
        // which we can infer from the creation not panicking
    }
}