pub mod command_registry;
pub mod config;
pub mod event_bus;
pub mod gpu_mock;
pub mod shared_state;
pub mod theme;
pub mod voice_mock;
pub mod wizard;

pub use command_registry::CommandRegistry;
pub use config::Config;
pub use event_bus::{AppEvent, EventBus};
pub use gpu_mock::GpuMock;
pub use shared_state::SharedAppState;
pub use theme::{Theme, ThemePalette};
pub use voice_mock::VoiceMock;
pub use wizard::Wizard;

pub fn hello() -> &'static str {
    "engine ok"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smoke() {
        assert_eq!(hello(), "engine ok");
    }
}
