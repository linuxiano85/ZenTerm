pub mod config;
pub mod event_bus;
pub mod command_registry;
pub mod gpu_mock;
pub mod theme;
pub mod voice_mock;
pub mod shared_state;
pub mod wizard;

pub use config::Config;
pub use event_bus::{EventBus, AppEvent};
pub use command_registry::CommandRegistry;
pub use gpu_mock::GpuMock;
pub use theme::{Theme, ThemePalette};
pub use voice_mock::VoiceMock;
pub use shared_state::SharedAppState;
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
