use log::info;

/// Multi-step setup wizard for first-run configuration
#[derive(Debug, Clone)]
pub struct Wizard {
    is_open: bool,
    current_step: WizardStep,
    completed_steps: Vec<WizardStep>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WizardStep {
    Welcome,
    GpuConfiguration,
    ThemeSelection,
    VoiceSetup,
    Complete,
}

#[derive(Debug, Clone)]
pub struct WizardStepData {
    pub title: String,
    pub description: String,
    pub can_skip: bool,
}

impl Wizard {
    /// Create a new wizard instance
    pub fn new() -> Self {
        Self {
            is_open: false,
            current_step: WizardStep::Welcome,
            completed_steps: Vec::new(),
        }
    }

    /// Open the wizard
    pub fn open(&mut self) {
        info!("Opening setup wizard");
        self.is_open = true;
        self.current_step = WizardStep::Welcome;
        self.completed_steps.clear();
    }

    /// Close the wizard
    pub fn close(&mut self) {
        info!("Closing setup wizard");
        self.is_open = false;
    }

    /// Check if wizard is open
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Get current wizard step
    pub fn current_step(&self) -> WizardStep {
        self.current_step.clone()
    }

    /// Go to the next step
    pub fn next_step(&mut self) -> bool {
        if !self.completed_steps.contains(&self.current_step) {
            self.completed_steps.push(self.current_step.clone());
        }

        let next_step = match self.current_step {
            WizardStep::Welcome => WizardStep::GpuConfiguration,
            WizardStep::GpuConfiguration => WizardStep::ThemeSelection,
            WizardStep::ThemeSelection => WizardStep::VoiceSetup,
            WizardStep::VoiceSetup => WizardStep::Complete,
            WizardStep::Complete => {
                self.close();
                return false; // No more steps
            }
        };

        self.current_step = next_step;
        true
    }

    /// Go to the previous step
    pub fn previous_step(&mut self) -> bool {
        let prev_step = match self.current_step {
            WizardStep::Welcome => return false, // Can't go back from welcome
            WizardStep::GpuConfiguration => WizardStep::Welcome,
            WizardStep::ThemeSelection => WizardStep::GpuConfiguration,
            WizardStep::VoiceSetup => WizardStep::ThemeSelection,
            WizardStep::Complete => WizardStep::VoiceSetup,
        };

        self.current_step = prev_step;
        true
    }

    /// Skip the current step (if allowed)
    pub fn skip_step(&mut self) -> bool {
        let step_data = self.get_step_data(&self.current_step);
        if step_data.can_skip {
            self.next_step()
        } else {
            false
        }
    }

    /// Get data for a specific wizard step
    pub fn get_step_data(&self, step: &WizardStep) -> WizardStepData {
        match step {
            WizardStep::Welcome => WizardStepData {
                title: "Welcome to ZenTerm".to_string(),
                description: "Welcome to ZenTerm Birthday MVP! This wizard will help you configure the application for first use. You can change these settings later in the application.".to_string(),
                can_skip: false,
            },
            WizardStep::GpuConfiguration => WizardStepData {
                title: "GPU Configuration".to_string(),
                description: "Set your preferred GPU usage limit. This helps manage system resources and performance. You can choose from 25%, 50%, 75%, or 100% GPU utilization.".to_string(),
                can_skip: true,
            },
            WizardStep::ThemeSelection => WizardStepData {
                title: "Theme Selection".to_string(),
                description: "Choose your preferred visual theme. Dark mode is easier on the eyes in low-light conditions, while light mode provides better contrast in bright environments.".to_string(),
                can_skip: true,
            },
            WizardStep::VoiceSetup => WizardStepData {
                title: "Voice Recognition".to_string(),
                description: "Enable voice recognition for hands-free terminal control. Note: This is currently a mock implementation. Real voice recognition will be added in future releases.".to_string(),
                can_skip: true,
            },
            WizardStep::Complete => WizardStepData {
                title: "Setup Complete".to_string(),
                description: "Congratulations! Your ZenTerm setup is complete. You can access this wizard again from the sidebar or change any settings through the application interface.".to_string(),
                can_skip: false,
            },
        }
    }

    /// Get current step data
    pub fn current_step_data(&self) -> WizardStepData {
        self.get_step_data(&self.current_step)
    }

    /// Check if a step has been completed
    pub fn is_step_completed(&self, step: &WizardStep) -> bool {
        self.completed_steps.contains(step)
    }

    /// Get wizard progress (0.0 to 1.0)
    pub fn progress(&self) -> f32 {
        let total_steps = 5; // Welcome, GPU, Theme, Voice, Complete
        let current_index = match self.current_step {
            WizardStep::Welcome => 0,
            WizardStep::GpuConfiguration => 1,
            WizardStep::ThemeSelection => 2,
            WizardStep::VoiceSetup => 3,
            WizardStep::Complete => 4,
        };

        current_index as f32 / (total_steps - 1) as f32
    }

    /// Check if wizard can go to next step
    pub fn can_go_next(&self) -> bool {
        !matches!(self.current_step, WizardStep::Complete)
    }

    /// Check if wizard can go to previous step
    pub fn can_go_previous(&self) -> bool {
        !matches!(self.current_step, WizardStep::Welcome)
    }

    /// Get total number of steps
    pub fn total_steps(&self) -> usize {
        5
    }

    /// Get current step index (0-based)
    pub fn current_step_index(&self) -> usize {
        match self.current_step {
            WizardStep::Welcome => 0,
            WizardStep::GpuConfiguration => 1,
            WizardStep::ThemeSelection => 2,
            WizardStep::VoiceSetup => 3,
            WizardStep::Complete => 4,
        }
    }
}

impl Default for Wizard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wizard_creation() {
        let wizard = Wizard::new();
        assert!(!wizard.is_open());
        assert_eq!(wizard.current_step(), WizardStep::Welcome);
        assert!(wizard.completed_steps.is_empty());
    }

    #[test]
    fn test_wizard_open_close() {
        let mut wizard = Wizard::new();

        wizard.open();
        assert!(wizard.is_open());
        assert_eq!(wizard.current_step(), WizardStep::Welcome);

        wizard.close();
        assert!(!wizard.is_open());
    }

    #[test]
    fn test_wizard_step_progression() {
        let mut wizard = Wizard::new();
        wizard.open();

        // Start at Welcome
        assert_eq!(wizard.current_step(), WizardStep::Welcome);
        assert!(!wizard.can_go_previous());
        assert!(wizard.can_go_next());

        // Go to GPU Configuration
        assert!(wizard.next_step());
        assert_eq!(wizard.current_step(), WizardStep::GpuConfiguration);
        assert!(wizard.can_go_previous());
        assert!(wizard.can_go_next());

        // Go to Theme Selection
        assert!(wizard.next_step());
        assert_eq!(wizard.current_step(), WizardStep::ThemeSelection);

        // Go to Voice Setup
        assert!(wizard.next_step());
        assert_eq!(wizard.current_step(), WizardStep::VoiceSetup);

        // Go to Complete
        assert!(wizard.next_step());
        assert_eq!(wizard.current_step(), WizardStep::Complete);
        assert!(!wizard.can_go_next());

        // Next step from Complete should close wizard
        assert!(!wizard.next_step());
        assert!(!wizard.is_open());
    }

    #[test]
    fn test_wizard_backward_progression() {
        let mut wizard = Wizard::new();
        wizard.open();

        // Go forward to Theme Selection
        wizard.next_step(); // GPU
        wizard.next_step(); // Theme
        assert_eq!(wizard.current_step(), WizardStep::ThemeSelection);

        // Go back to GPU Configuration
        assert!(wizard.previous_step());
        assert_eq!(wizard.current_step(), WizardStep::GpuConfiguration);

        // Go back to Welcome
        assert!(wizard.previous_step());
        assert_eq!(wizard.current_step(), WizardStep::Welcome);

        // Can't go back further
        assert!(!wizard.previous_step());
        assert_eq!(wizard.current_step(), WizardStep::Welcome);
    }

    #[test]
    fn test_wizard_step_data() {
        let wizard = Wizard::new();

        let welcome_data = wizard.get_step_data(&WizardStep::Welcome);
        assert_eq!(welcome_data.title, "Welcome to ZenTerm");
        assert!(!welcome_data.can_skip);

        let gpu_data = wizard.get_step_data(&WizardStep::GpuConfiguration);
        assert!(gpu_data.title.contains("GPU"));
        assert!(gpu_data.can_skip);

        let theme_data = wizard.get_step_data(&WizardStep::ThemeSelection);
        assert!(theme_data.title.contains("Theme"));
        assert!(theme_data.can_skip);

        let voice_data = wizard.get_step_data(&WizardStep::VoiceSetup);
        assert!(voice_data.title.contains("Voice"));
        assert!(voice_data.can_skip);

        let complete_data = wizard.get_step_data(&WizardStep::Complete);
        assert!(complete_data.title.contains("Complete"));
        assert!(!complete_data.can_skip);
    }

    #[test]
    fn test_wizard_progress() {
        let mut wizard = Wizard::new();
        wizard.open();

        assert_eq!(wizard.progress(), 0.0); // Welcome step

        wizard.next_step();
        assert_eq!(wizard.progress(), 0.25); // GPU step

        wizard.next_step();
        assert_eq!(wizard.progress(), 0.5); // Theme step

        wizard.next_step();
        assert_eq!(wizard.progress(), 0.75); // Voice step

        wizard.next_step();
        assert_eq!(wizard.progress(), 1.0); // Complete step
    }

    #[test]
    fn test_wizard_step_completion_tracking() {
        let mut wizard = Wizard::new();
        wizard.open();

        assert!(!wizard.is_step_completed(&WizardStep::Welcome));

        wizard.next_step(); // Complete Welcome, move to GPU
        assert!(wizard.is_step_completed(&WizardStep::Welcome));
        assert!(!wizard.is_step_completed(&WizardStep::GpuConfiguration));

        wizard.next_step(); // Complete GPU, move to Theme
        assert!(wizard.is_step_completed(&WizardStep::GpuConfiguration));
        assert!(!wizard.is_step_completed(&WizardStep::ThemeSelection));
    }

    #[test]
    fn test_wizard_step_skipping() {
        let mut wizard = Wizard::new();
        wizard.open();

        // Can't skip Welcome step
        assert!(!wizard.skip_step());
        assert_eq!(wizard.current_step(), WizardStep::Welcome);

        // Move to GPU step and skip it
        wizard.next_step();
        assert_eq!(wizard.current_step(), WizardStep::GpuConfiguration);
        assert!(wizard.skip_step()); // Should move to Theme step
        assert_eq!(wizard.current_step(), WizardStep::ThemeSelection);
    }
}
