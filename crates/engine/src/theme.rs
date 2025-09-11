/// Theme management for the application
#[derive(Debug, Clone)]
pub struct Theme {
    pub dark_mode: bool,
    pub palette: ThemePalette,
}

/// Color palette for the application theme
#[derive(Debug, Clone)]
pub struct ThemePalette {
    // Background colors
    pub background_primary: [u8; 3],
    pub background_secondary: [u8; 3],
    pub background_tertiary: [u8; 3],

    // Text colors
    pub text_primary: [u8; 3],
    pub text_secondary: [u8; 3],
    pub text_muted: [u8; 3],

    // Accent colors
    pub accent_primary: [u8; 3],
    pub accent_secondary: [u8; 3],

    // Status colors
    pub success: [u8; 3],
    pub warning: [u8; 3],
    pub error: [u8; 3],

    // UI element colors
    pub border: [u8; 3],
    pub separator: [u8; 3],
    pub selection: [u8; 3],
}

impl Theme {
    /// Create a new theme with the specified mode
    pub fn new(dark_mode: bool) -> Self {
        Self {
            dark_mode,
            palette: if dark_mode {
                ThemePalette::dark()
            } else {
                ThemePalette::light()
            },
        }
    }

    /// Toggle between light and dark mode
    pub fn toggle(&mut self) {
        self.dark_mode = !self.dark_mode;
        self.palette = if self.dark_mode {
            ThemePalette::dark()
        } else {
            ThemePalette::light()
        };
    }

    /// Get the current theme name
    pub fn name(&self) -> &'static str {
        if self.dark_mode {
            "Dark"
        } else {
            "Light"
        }
    }

    /// Apply theme to egui context (helper for GUI integration)
    pub fn apply_to_egui(&self, ctx: &egui::Context) {
        let visuals = if self.dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };
        ctx.set_visuals(visuals);
    }
}

impl ThemePalette {
    /// Create a dark theme palette
    pub fn dark() -> Self {
        Self {
            // Dark backgrounds
            background_primary: [33, 37, 43],   // #212529
            background_secondary: [52, 58, 64], // #343a40
            background_tertiary: [73, 80, 87],  // #495057

            // Light text on dark background
            text_primary: [248, 249, 250],   // #f8f9fa
            text_secondary: [222, 226, 230], // #dee2e6
            text_muted: [173, 181, 189],     // #adb5bd

            // Accent colors (blue/cyan theme)
            accent_primary: [13, 202, 240],   // #0dcaf0 (cyan)
            accent_secondary: [111, 66, 193], // #6f42c1 (purple)

            // Status colors
            success: [25, 135, 84], // #198754 (green)
            warning: [255, 193, 7], // #ffc107 (yellow)
            error: [220, 53, 69],   // #dc3545 (red)

            // UI elements
            border: [73, 80, 87],       // #495057
            separator: [108, 117, 125], // #6c757d
            selection: [13, 202, 240],  // #0dcaf0 (cyan)
        }
    }

    /// Create a light theme palette
    pub fn light() -> Self {
        Self {
            // Light backgrounds
            background_primary: [255, 255, 255],   // #ffffff
            background_secondary: [248, 249, 250], // #f8f9fa
            background_tertiary: [233, 236, 239],  // #e9ecef

            // Dark text on light background
            text_primary: [33, 37, 43],   // #212529
            text_secondary: [73, 80, 87], // #495057
            text_muted: [108, 117, 125],  // #6c757d

            // Accent colors (blue/purple theme)
            accent_primary: [13, 110, 253],   // #0d6efd (blue)
            accent_secondary: [111, 66, 193], // #6f42c1 (purple)

            // Status colors
            success: [25, 135, 84], // #198754 (green)
            warning: [255, 193, 7], // #ffc107 (yellow)
            error: [220, 53, 69],   // #dc3545 (red)

            // UI elements
            border: [222, 226, 230],    // #dee2e6
            separator: [173, 181, 189], // #adb5bd
            selection: [13, 110, 253],  // #0d6efd (blue)
        }
    }

    /// Convert RGB array to hex string for debugging
    pub fn rgb_to_hex(rgb: [u8; 3]) -> String {
        format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2])
    }

    /// Convert RGB array to normalized float array for graphics APIs
    pub fn rgb_to_float(rgb: [u8; 3]) -> [f32; 3] {
        [
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0,
        ]
    }

    /// Convert RGB array to egui Color32
    pub fn rgb_to_color32(rgb: [u8; 3]) -> egui::Color32 {
        egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2])
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new(true) // Default to dark mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let dark_theme = Theme::new(true);
        assert!(dark_theme.dark_mode);
        assert_eq!(dark_theme.name(), "Dark");

        let light_theme = Theme::new(false);
        assert!(!light_theme.dark_mode);
        assert_eq!(light_theme.name(), "Light");
    }

    #[test]
    fn test_theme_toggle() {
        let mut theme = Theme::new(true);
        assert!(theme.dark_mode);

        theme.toggle();
        assert!(!theme.dark_mode);
        assert_eq!(theme.name(), "Light");

        theme.toggle();
        assert!(theme.dark_mode);
        assert_eq!(theme.name(), "Dark");
    }

    #[test]
    fn test_theme_palette_colors() {
        let dark_palette = ThemePalette::dark();
        let light_palette = ThemePalette::light();

        // Dark and light palettes should have different colors
        assert_ne!(
            dark_palette.background_primary,
            light_palette.background_primary
        );
        assert_ne!(dark_palette.text_primary, light_palette.text_primary);

        // But some colors like success/warning/error should be the same
        assert_eq!(dark_palette.success, light_palette.success);
        assert_eq!(dark_palette.warning, light_palette.warning);
        assert_eq!(dark_palette.error, light_palette.error);
    }

    #[test]
    fn test_rgb_conversions() {
        let rgb = [255, 128, 64];

        // Test hex conversion
        assert_eq!(ThemePalette::rgb_to_hex(rgb), "#ff8040");

        // Test float conversion
        let float_rgb = ThemePalette::rgb_to_float(rgb);
        assert_eq!(float_rgb[0], 1.0);
        assert!((float_rgb[1] - 0.502).abs() < 0.01); // ~128/255
        assert!((float_rgb[2] - 0.251).abs() < 0.01); // ~64/255
    }

    #[test]
    fn test_default_theme() {
        let theme = Theme::default();
        assert!(theme.dark_mode);
        assert_eq!(theme.name(), "Dark");
    }
}
