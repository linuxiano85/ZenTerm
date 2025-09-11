pub mod config;
pub mod gpu;
pub mod ui;

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
