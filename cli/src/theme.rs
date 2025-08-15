use ratatui::style::Color;
use rusty2048_shared::Theme;

/// Convert hex color string to ratatui Color
pub fn hex_to_color(hex: &str) -> Color {
    if hex.starts_with('#') && hex.len() == 7 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[1..3], 16),
            u8::from_str_radix(&hex[3..5], 16),
            u8::from_str_radix(&hex[5..7], 16),
        ) {
            return Color::Rgb(r, g, b);
        }
    }
    Color::White // fallback
}

/// Get tile color based on value and theme
pub fn get_tile_color(value: u32, theme: &Theme) -> Color {
    if value == 0 {
        return hex_to_color(&theme.tile_colors[0]);
    }
    
    let color_index = value.trailing_zeros() as usize;
    if color_index < theme.tile_colors.len() {
        hex_to_color(&theme.tile_colors[color_index])
    } else {
        // For values beyond our color palette, cycle through colors
        let index = (color_index - theme.tile_colors.len()) % (theme.tile_colors.len() - 1) + 1;
        hex_to_color(&theme.tile_colors[index])
    }
}

/// Get text color for tile based on value and theme
pub fn get_tile_text_color(value: u32, theme: &Theme) -> Color {
    if value == 0 {
        return hex_to_color(&theme.text_color);
    }
    
    // For dark tiles, use light text; for light tiles, use dark text
    let tile_color = get_tile_color(value, theme);
    match tile_color {
        Color::Rgb(r, g, b) => {
            // Calculate luminance
            let luminance = (0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64) / 255.0;
            if luminance > 0.5 {
                Color::Black
            } else {
                Color::White
            }
        }
        _ => hex_to_color(&theme.text_color),
    }
}

/// Theme manager for CLI
pub struct ThemeManager {
    pub current_theme: Theme,
    pub themes: Vec<Theme>,
    pub current_index: usize,
}

impl ThemeManager {
    pub fn new() -> Self {
        let themes = Theme::all_themes();
        Self {
            current_theme: themes[0].clone(),
            themes,
            current_index: 0,
        }
    }
    
    /// Switch to next theme
    pub fn next_theme(&mut self) {
        self.current_index = (self.current_index + 1) % self.themes.len();
        self.current_theme = self.themes[self.current_index].clone();
    }
    
    /// Switch to previous theme
    #[allow(dead_code)]
    pub fn prev_theme(&mut self) {
        self.current_index = if self.current_index == 0 {
            self.themes.len() - 1
        } else {
            self.current_index - 1
        };
        self.current_theme = self.themes[self.current_index].clone();
    }
    
    /// Switch to specific theme by name
    pub fn set_theme(&mut self, name: &str) -> bool {
        if let Some(theme) = Theme::by_name(name) {
            self.current_theme = theme;
            self.current_index = self.themes.iter().position(|t| t.name == name).unwrap_or(0);
            true
        } else {
            false
        }
    }
    
    /// Get current theme name
    pub fn current_theme_name(&self) -> &str {
        &self.current_theme.name
    }
    
    /// Get all theme names
    #[allow(dead_code)]
    pub fn theme_names(&self) -> Vec<&str> {
        self.themes.iter().map(|t| t.name.as_str()).collect()
    }
}
