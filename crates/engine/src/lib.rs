pub mod ui;

pub fn hello() -> &'static str {
    "engine ok"
}

// Re-export the main TUI functions for easy access
pub use ui::{install_panic_guard, run_tui};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smoke() {
        assert_eq!(hello(), "engine ok");
    }
}
