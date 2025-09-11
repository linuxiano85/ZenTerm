use log::{info, warn};
use std::collections::HashMap;

/// Command registry for managing application commands
/// This is a placeholder for future command palette integration
#[derive(Debug, Clone)]
pub struct CommandRegistry {
    commands: HashMap<String, Command>,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub category: CommandCategory,
}

#[derive(Debug, Clone)]
pub enum CommandCategory {
    Gpu,
    Theme,
    Voice,
    Wizard,
    System,
}

impl CommandRegistry {
    /// Create a new command registry
    pub fn new() -> Self {
        let mut registry = Self {
            commands: HashMap::new(),
        };

        // Register default commands
        registry.register_default_commands();
        registry
    }

    /// Register a new command
    pub fn register(&mut self, id: String, command: Command) {
        info!("Registering command: {} - {}", id, command.name);
        self.commands.insert(id, command);
    }

    /// Get a command by ID
    pub fn get(&self, id: &str) -> Option<&Command> {
        self.commands.get(id)
    }

    /// Get all commands
    pub fn all_commands(&self) -> Vec<(&String, &Command)> {
        self.commands.iter().collect()
    }

    /// Get commands by category
    pub fn commands_by_category(&self, category: &CommandCategory) -> Vec<(&String, &Command)> {
        self.commands
            .iter()
            .filter(|(_, cmd)| &cmd.category == category)
            .collect()
    }

    /// Execute a command by ID (placeholder implementation)
    pub fn execute(&self, id: &str) -> Result<(), String> {
        match self.commands.get(id) {
            Some(command) => {
                info!("Executing command: {}", command.name);
                // TODO: Implement actual command execution
                Ok(())
            }
            None => {
                warn!("Command not found: {}", id);
                Err(format!("Command '{}' not found", id))
            }
        }
    }

    /// Register default commands for the application
    fn register_default_commands(&mut self) {
        let commands = vec![
            (
                "gpu.limit.25",
                Command {
                    name: "Set GPU Limit to 25%".to_string(),
                    description: "Limit GPU usage to 25%".to_string(),
                    category: CommandCategory::Gpu,
                },
            ),
            (
                "gpu.limit.50",
                Command {
                    name: "Set GPU Limit to 50%".to_string(),
                    description: "Limit GPU usage to 50%".to_string(),
                    category: CommandCategory::Gpu,
                },
            ),
            (
                "gpu.limit.75",
                Command {
                    name: "Set GPU Limit to 75%".to_string(),
                    description: "Limit GPU usage to 75%".to_string(),
                    category: CommandCategory::Gpu,
                },
            ),
            (
                "gpu.limit.100",
                Command {
                    name: "Set GPU Limit to 100%".to_string(),
                    description: "Use full GPU capacity".to_string(),
                    category: CommandCategory::Gpu,
                },
            ),
            (
                "theme.toggle",
                Command {
                    name: "Toggle Theme".to_string(),
                    description: "Switch between light and dark theme".to_string(),
                    category: CommandCategory::Theme,
                },
            ),
            (
                "voice.toggle",
                Command {
                    name: "Toggle Voice".to_string(),
                    description: "Enable or disable voice recognition".to_string(),
                    category: CommandCategory::Voice,
                },
            ),
            (
                "wizard.open",
                Command {
                    name: "Open Setup Wizard".to_string(),
                    description: "Launch the initial setup wizard".to_string(),
                    category: CommandCategory::Wizard,
                },
            ),
            (
                "system.quit",
                Command {
                    name: "Quit Application".to_string(),
                    description: "Exit ZenTerm".to_string(),
                    category: CommandCategory::System,
                },
            ),
        ];

        for (id, command) in commands {
            self.register(id.to_string(), command);
        }
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Implement PartialEq for CommandCategory for testing
impl PartialEq for CommandCategory {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (CommandCategory::Gpu, CommandCategory::Gpu)
                | (CommandCategory::Theme, CommandCategory::Theme)
                | (CommandCategory::Voice, CommandCategory::Voice)
                | (CommandCategory::Wizard, CommandCategory::Wizard)
                | (CommandCategory::System, CommandCategory::System)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_registry_creation() {
        let registry = CommandRegistry::new();

        // Should have default commands registered
        assert!(!registry.commands.is_empty());

        // Check for some expected commands
        assert!(registry.get("gpu.limit.25").is_some());
        assert!(registry.get("theme.toggle").is_some());
        assert!(registry.get("voice.toggle").is_some());
        assert!(registry.get("wizard.open").is_some());
        assert!(registry.get("system.quit").is_some());
    }

    #[test]
    fn test_command_registration() {
        let mut registry = CommandRegistry::new();
        let initial_count = registry.commands.len();

        let custom_command = Command {
            name: "Test Command".to_string(),
            description: "A test command".to_string(),
            category: CommandCategory::System,
        };

        registry.register("test.command".to_string(), custom_command);

        assert_eq!(registry.commands.len(), initial_count + 1);
        assert!(registry.get("test.command").is_some());
    }

    #[test]
    fn test_commands_by_category() {
        let registry = CommandRegistry::new();

        let gpu_commands = registry.commands_by_category(&CommandCategory::Gpu);
        assert!(!gpu_commands.is_empty());

        // Verify all returned commands are actually GPU category
        for (_, command) in gpu_commands {
            assert_eq!(command.category, CommandCategory::Gpu);
        }
    }

    #[test]
    fn test_command_execution() {
        let registry = CommandRegistry::new();

        // Test executing existing command
        assert!(registry.execute("gpu.limit.25").is_ok());

        // Test executing non-existent command
        assert!(registry.execute("nonexistent.command").is_err());
    }
}
