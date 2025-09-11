use ratatui::style::Color;

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub theme: Theme,
    pub accent: Color,
    pub border: Color,
    pub border_accent: Color,
    pub highlight: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub background: Color,
    pub surface: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
}

impl Palette {
    pub fn dark() -> Self {
        Self {
            theme: Theme::Dark,
            accent: Color::Rgb(99, 102, 241),      // Indigo-500
            border: Color::Rgb(55, 65, 81),       // Gray-700
            border_accent: Color::Rgb(139, 92, 246), // Violet-500
            highlight: Color::Rgb(59, 130, 246),   // Blue-500
            text_primary: Color::Rgb(249, 250, 251), // Gray-50
            text_secondary: Color::Rgb(156, 163, 175), // Gray-400
            background: Color::Rgb(17, 24, 39),    // Gray-900
            surface: Color::Rgb(31, 41, 55),       // Gray-800
            success: Color::Rgb(34, 197, 94),      // Green-500
            warning: Color::Rgb(245, 158, 11),     // Amber-500
            error: Color::Rgb(239, 68, 68),        // Red-500
        }
    }

    pub fn light() -> Self {
        Self {
            theme: Theme::Light,
            accent: Color::Rgb(99, 102, 241),      // Indigo-500
            border: Color::Rgb(209, 213, 219),     // Gray-300
            border_accent: Color::Rgb(139, 92, 246), // Violet-500
            highlight: Color::Rgb(59, 130, 246),   // Blue-500
            text_primary: Color::Rgb(17, 24, 39),  // Gray-900
            text_secondary: Color::Rgb(75, 85, 99), // Gray-600
            background: Color::Rgb(255, 255, 255), // White
            surface: Color::Rgb(249, 250, 251),    // Gray-50
            success: Color::Rgb(34, 197, 94),      // Green-500
            warning: Color::Rgb(245, 158, 11),     // Amber-500
            error: Color::Rgb(239, 68, 68),        // Red-500
        }
    }

    pub fn toggle_theme(&mut self) {
        *self = match self.theme {
            Theme::Dark => Self::light(),
            Theme::Light => Self::dark(),
        };
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::dark()
    }
}