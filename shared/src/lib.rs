//! Shared components for Rusty2048
//! 
//! This module contains shared utilities, themes, and components
//! that can be used across different platforms.

use serde::{Deserialize, Serialize};

/// Color theme for the game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background: String,
    pub grid_background: String,
    pub tile_colors: Vec<String>,
    pub text_color: String,
    pub title_color: String,
    pub score_color: String,
    pub best_score_color: String,
    pub moves_color: String,
    pub time_color: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "Classic".to_string(),
            background: "#faf8ef".to_string(),
            grid_background: "#bbada0".to_string(),
            tile_colors: vec![
                "#cdc1b4".to_string(), // 0
                "#eee4da".to_string(), // 2
                "#ede0c8".to_string(), // 4
                "#f2b179".to_string(), // 8
                "#f59563".to_string(), // 16
                "#f67c5f".to_string(), // 32
                "#f65e3b".to_string(), // 64
                "#edcf72".to_string(), // 128
                "#edcc61".to_string(), // 256
                "#edc850".to_string(), // 512
                "#edc53f".to_string(), // 1024
                "#edc22e".to_string(), // 2048
            ],
            text_color: "#776e65".to_string(),
            title_color: "#776e65".to_string(),
            score_color: "#776e65".to_string(),
            best_score_color: "#776e65".to_string(),
            moves_color: "#776e65".to_string(),
            time_color: "#776e65".to_string(),
        }
    }
}

impl Theme {
    /// Create a dark theme
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            background: "#1a1a1a".to_string(),
            grid_background: "#2d2d2d".to_string(),
            tile_colors: vec![
                "#3c3c3c".to_string(), // 0
                "#4a4a4a".to_string(), // 2
                "#5a5a5a".to_string(), // 4
                "#6a6a6a".to_string(), // 8
                "#7a7a7a".to_string(), // 16
                "#8a8a8a".to_string(), // 32
                "#9a9a9a".to_string(), // 64
                "#aaaaaa".to_string(), // 128
                "#bbbbbb".to_string(), // 256
                "#cccccc".to_string(), // 512
                "#dddddd".to_string(), // 1024
                "#eeeeee".to_string(), // 2048
            ],
            text_color: "#ffffff".to_string(),
            title_color: "#ffffff".to_string(),
            score_color: "#4ade80".to_string(),
            best_score_color: "#fbbf24".to_string(),
            moves_color: "#60a5fa".to_string(),
            time_color: "#a78bfa".to_string(),
        }
    }

    /// Create a neon theme
    pub fn neon() -> Self {
        Self {
            name: "Neon".to_string(),
            background: "#000000".to_string(),
            grid_background: "#1a0033".to_string(),
            tile_colors: vec![
                "#330033".to_string(), // 0
                "#ff00ff".to_string(), // 2
                "#00ffff".to_string(), // 4
                "#ffff00".to_string(), // 8
                "#ff0080".to_string(), // 16
                "#80ff00".to_string(), // 32
                "#0080ff".to_string(), // 64
                "#ff8000".to_string(), // 128
                "#8000ff".to_string(), // 256
                "#00ff80".to_string(), // 512
                "#ff0080".to_string(), // 1024
                "#ffff00".to_string(), // 2048
            ],
            text_color: "#ffffff".to_string(),
            title_color: "#ff00ff".to_string(),
            score_color: "#00ffff".to_string(),
            best_score_color: "#ffff00".to_string(),
            moves_color: "#ff0080".to_string(),
            time_color: "#80ff00".to_string(),
        }
    }

    /// Create a retro theme
    pub fn retro() -> Self {
        Self {
            name: "Retro".to_string(),
            background: "#2b2b2b".to_string(),
            grid_background: "#404040".to_string(),
            tile_colors: vec![
                "#555555".to_string(), // 0
                "#00ff00".to_string(), // 2
                "#00dd00".to_string(), // 4
                "#00bb00".to_string(), // 8
                "#009900".to_string(), // 16
                "#007700".to_string(), // 32
                "#005500".to_string(), // 64
                "#003300".to_string(), // 128
                "#001100".to_string(), // 256
                "#00ff00".to_string(), // 512
                "#00dd00".to_string(), // 1024
                "#00bb00".to_string(), // 2048
            ],
            text_color: "#00ff00".to_string(),
            title_color: "#00ff00".to_string(),
            score_color: "#00ff00".to_string(),
            best_score_color: "#00ff00".to_string(),
            moves_color: "#00ff00".to_string(),
            time_color: "#00ff00".to_string(),
        }
    }

    /// Create a pastel theme
    pub fn pastel() -> Self {
        Self {
            name: "Pastel".to_string(),
            background: "#f8f9fa".to_string(),
            grid_background: "#e9ecef".to_string(),
            tile_colors: vec![
                "#dee2e6".to_string(), // 0
                "#ffb3ba".to_string(), // 2
                "#baffc9".to_string(), // 4
                "#bae1ff".to_string(), // 8
                "#ffffba".to_string(), // 16
                "#ffb3d9".to_string(), // 32
                "#d9b3ff".to_string(), // 64
                "#b3d9ff".to_string(), // 128
                "#b3ffd9".to_string(), // 256
                "#ffd9b3".to_string(), // 512
                "#d9ffb3".to_string(), // 1024
                "#ffb3b3".to_string(), // 2048
            ],
            text_color: "#495057".to_string(),
            title_color: "#6c757d".to_string(),
            score_color: "#28a745".to_string(),
            best_score_color: "#ffc107".to_string(),
            moves_color: "#17a2b8".to_string(),
            time_color: "#6f42c1".to_string(),
        }
    }

    /// Get all available themes
    pub fn all_themes() -> Vec<Self> {
        vec![
            Self::default(),
            Self::dark(),
            Self::neon(),
            Self::retro(),
            Self::pastel(),
        ]
    }

    /// Get theme by name
    pub fn by_name(name: &str) -> Option<Self> {
        Self::all_themes().into_iter().find(|t| t.name == name)
    }
}

/// Game configuration that can be shared across platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedConfig {
    pub theme: Theme,
    pub board_size: usize,
    pub target_score: u32,
    pub enable_animations: bool,
    pub enable_sound: bool,
}

impl Default for SharedConfig {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            board_size: 4,
            target_score: 2048,
            enable_animations: true,
            enable_sound: false,
        }
    }
}

/// Animation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    pub duration_ms: u32,
    pub easing: String,
    pub enable_slide: bool,
    pub enable_merge: bool,
    pub enable_spawn: bool,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            duration_ms: 150,
            easing: "ease-out".to_string(),
            enable_slide: true,
            enable_merge: true,
            enable_spawn: true,
        }
    }
}
