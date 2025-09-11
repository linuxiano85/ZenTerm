use log::{debug, info};

/// Mock GPU controller for simulating GPU usage limits
/// This is a placeholder for future real GPU control integration
#[derive(Debug, Clone)]
pub struct GpuMock {
    current_limit: u8,
    current_usage: f32, // Simulated current usage percentage
}

impl GpuMock {
    /// Create a new GPU mock controller
    pub fn new() -> Self {
        Self {
            current_limit: 75,   // Default limit
            current_usage: 45.0, // Simulated initial usage
        }
    }

    /// Set the GPU usage limit (25, 50, 75, or 100)
    pub fn set_limit(&mut self, limit: u8) -> Result<(), String> {
        match limit {
            25 | 50 | 75 | 100 => {
                info!("Setting GPU limit to {}%", limit);
                self.current_limit = limit;

                // Simulate usage adjustment based on new limit
                if self.current_usage > limit as f32 {
                    self.current_usage = limit as f32 * 0.9; // Use 90% of limit
                    debug!(
                        "Adjusted GPU usage to {:.1}% due to new limit",
                        self.current_usage
                    );
                }

                Ok(())
            }
            _ => Err(format!(
                "Invalid GPU limit: {}. Must be 25, 50, 75, or 100",
                limit
            )),
        }
    }

    /// Get the current GPU usage limit
    pub fn get_limit(&self) -> u8 {
        self.current_limit
    }

    /// Get the current simulated GPU usage percentage
    pub fn get_usage(&self) -> f32 {
        self.current_usage
    }

    /// Simulate GPU usage fluctuation (for demonstration purposes)
    pub fn update_usage(&mut self) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::{SystemTime, UNIX_EPOCH};

        // Simple pseudo-random fluctuation based on current time
        let mut hasher = DefaultHasher::new();
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .hash(&mut hasher);

        let random_factor = (hasher.finish() % 1000) as f32 / 1000.0; // 0.0 to 1.0

        // Create realistic fluctuation (±10% of current usage)
        let fluctuation = (random_factor - 0.5) * 0.2 * self.current_usage;
        let new_usage = (self.current_usage + fluctuation)
            .max(10.0) // Minimum 10% usage
            .min(self.current_limit as f32); // Don't exceed limit

        self.current_usage = new_usage;
    }

    /// Get GPU status as a formatted string
    pub fn status_string(&self) -> String {
        format!("{:.0}%", self.current_usage)
    }

    /// Apply the GPU limit (placeholder for actual GPU control)
    pub fn apply_limit(&self) -> Result<(), String> {
        info!(
            "Applying GPU limit: {}% (current usage: {:.1}%)",
            self.current_limit, self.current_usage
        );

        // TODO: In a real implementation, this would interface with actual GPU drivers
        // For now, just log the action
        debug!("GPU limit applied successfully (mock implementation)");

        Ok(())
    }

    /// Check if GPU usage is near the limit
    pub fn is_near_limit(&self) -> bool {
        self.current_usage >= (self.current_limit as f32 * 0.9)
    }

    /// Get available GPU headroom
    pub fn get_headroom(&self) -> f32 {
        (self.current_limit as f32 - self.current_usage).max(0.0)
    }
}

impl Default for GpuMock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_mock_creation() {
        let gpu = GpuMock::new();
        assert_eq!(gpu.get_limit(), 75);
        assert!(gpu.get_usage() > 0.0);
        assert!(gpu.get_usage() <= 100.0);
    }

    #[test]
    fn test_gpu_limit_setting() {
        let mut gpu = GpuMock::new();

        // Test valid limits
        assert!(gpu.set_limit(25).is_ok());
        assert_eq!(gpu.get_limit(), 25);

        assert!(gpu.set_limit(50).is_ok());
        assert_eq!(gpu.get_limit(), 50);

        assert!(gpu.set_limit(75).is_ok());
        assert_eq!(gpu.get_limit(), 75);

        assert!(gpu.set_limit(100).is_ok());
        assert_eq!(gpu.get_limit(), 100);

        // Test invalid limits
        assert!(gpu.set_limit(30).is_err());
        assert!(gpu.set_limit(0).is_err());
        assert!(gpu.set_limit(150).is_err());
    }

    #[test]
    fn test_gpu_usage_adjustment() {
        let mut gpu = GpuMock::new();

        // Set usage to a high value
        gpu.current_usage = 80.0;

        // Set a lower limit - usage should be adjusted down
        gpu.set_limit(50).unwrap();
        assert!(gpu.get_usage() <= 50.0);

        // Set a higher limit - usage should remain the same
        let usage_before = gpu.get_usage();
        gpu.set_limit(75).unwrap();
        assert_eq!(gpu.get_usage(), usage_before);
    }

    #[test]
    fn test_gpu_status_methods() {
        let mut gpu = GpuMock::new();
        gpu.current_usage = 80.0;
        gpu.set_limit(100).unwrap();

        // Test status string
        assert_eq!(gpu.status_string(), "80%");

        // Test near limit detection
        gpu.set_limit(100).unwrap();
        assert!(!gpu.is_near_limit()); // 80% usage with 100% limit should not be near limit

        // Test with a limit that would make it near limit
        gpu.current_usage = 95.0;
        gpu.set_limit(100).unwrap();
        assert!(gpu.is_near_limit()); // 95% usage with 100% limit should be near limit

        // Test headroom calculation
        let headroom = gpu.get_headroom();
        assert!(headroom >= 0.0);
        assert_eq!(headroom, 100.0 - gpu.get_usage());
    }

    #[test]
    fn test_gpu_apply_limit() {
        let gpu = GpuMock::new();
        assert!(gpu.apply_limit().is_ok());
    }
}
