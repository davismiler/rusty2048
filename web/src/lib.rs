use wasm_bindgen::prelude::*;
use rusty2048_core::{Game, GameConfig, Direction, GameState};
use rusty2048_shared::{I18n, Language, TranslationKey};
use serde::Serialize;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Rusty2048Web {
    game: Game,
    i18n: I18n,
    current_theme: String,
}

#[wasm_bindgen]
impl Rusty2048Web {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        
        let config = GameConfig::default();
        let game = Game::new(config).expect("Failed to create game");
        let mut i18n = I18n::new();
        
        // Try to detect browser language
        if let Some(window) = web_sys::window() {
            let navigator = window.navigator();
            if let Some(lang) = navigator.language() {
                if let Some(language) = Language::from_code(&lang) {
                    i18n.set_language(language);
                }
            }
        }
        
        Self { 
            game, 
            i18n,
            current_theme: "Classic".to_string(),
        }
    }
    
    /// Get current language
    pub fn get_language(&self) -> String {
        self.i18n.current_language().code().to_string()
    }
    
    /// Set language
    pub fn set_language(&mut self, language_code: &str) -> Result<(), JsValue> {
        if let Some(language) = Language::from_code(language_code) {
            self.i18n.set_language(language);
            Ok(())
        } else {
            Err(JsValue::from_str("Invalid language code"))
        }
    }
    
    /// Get supported languages
    pub fn get_supported_languages(&self) -> JsValue {
        let languages: Vec<String> = self.i18n.supported_languages()
            .iter()
            .map(|lang| lang.code().to_string())
            .collect();
        serde_wasm_bindgen::to_value(&languages).unwrap()
    }
    
    /// Get translation for a key
    pub fn get_translation(&self, key: &str) -> String {
        // Convert string key to TranslationKey enum
        let translation_key = match key {
            "score" => TranslationKey::Score,
            "best" => TranslationKey::Best,
            "moves" => TranslationKey::Moves,
            "time" => TranslationKey::Time,
            "new_game" => TranslationKey::NewGame,
            "undo" => TranslationKey::Undo,
            "game_over" => TranslationKey::GameOver,
            "congratulations" => TranslationKey::Congratulations,
            "you_won" => TranslationKey::YouWon,
            "press_r_to_restart" => TranslationKey::PressRToRestart,
            "continue_playing" => TranslationKey::ContinuePlaying,
            "controls" => TranslationKey::Controls,
            "move_tiles" => TranslationKey::MoveTiles,
            "restart" => TranslationKey::Restart,
            "undo_move" => TranslationKey::UndoMove,
            "cycle_theme" => TranslationKey::CycleTheme,
            "select_theme" => TranslationKey::SelectTheme,
            "theme_help" => TranslationKey::ThemeHelp,
            "replay_mode" => TranslationKey::ReplayMode,
            "statistics_charts" => TranslationKey::StatisticsCharts,
            "ai_mode" => TranslationKey::AIMode,
            "help" => TranslationKey::Help,
            "quit" => TranslationKey::Quit,
            "language" => TranslationKey::Help, // Use Help as placeholder for "Language"
            _ => TranslationKey::Help, // Default fallback
        };
        
        self.i18n.t(&translation_key)
    }

    pub fn new_game(&mut self) -> Result<(), JsValue> {
        self.game.new_game().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn make_move(&mut self, direction: &str) -> Result<bool, JsValue> {
        let dir = match direction {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => return Err(JsValue::from_str("Invalid direction")),
        };
        
        self.game.make_move(dir).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn get_board(&self) -> Vec<u32> {
        let board = self.game.board();
        let mut result = Vec::new();
        for row in 0..board.size() {
            for col in 0..board.size() {
                if let Ok(tile) = board.get_tile(row, col) {
                    result.push(tile.value);
                }
            }
        }
        result
    }

    pub fn get_score(&self) -> JsValue {
        let score = self.game.score();
        serde_wasm_bindgen::to_value(&score).unwrap()
    }

    pub fn get_state(&self) -> String {
        match self.game.state() {
            GameState::Playing => "playing".to_string(),
            GameState::Won => "won".to_string(),
            GameState::GameOver => "game_over".to_string(),
        }
    }

    pub fn get_moves(&self) -> u32 {
        self.game.moves()
    }

    pub fn undo(&mut self) -> Result<(), JsValue> {
        self.game.undo().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn set_theme(&mut self, theme_name: &str) -> Result<(), JsValue> {
        // Update current theme
        self.current_theme = theme_name.to_string();
        Ok(())
    }

    /// Get current theme information
    pub fn get_theme(&self) -> JsValue {
        #[derive(Serialize)]
        struct Theme {
            background: String,
            title_color: String,
            text_color: String,
            grid_background: String,
            tile_colors: Vec<String>,
        }
        
        // Get theme based on current theme name
        let theme = match self.current_theme.as_str() {
            "Dark" => Theme {
                background: "#1a1a1a".to_string(),
                title_color: "#ffffff".to_string(),
                text_color: "#cccccc".to_string(),
                grid_background: "#2d2d2d".to_string(),
                tile_colors: vec![
                    "#3c3c3c".to_string(), // empty
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
            },
            "Neon" => Theme {
                background: "#000000".to_string(),
                title_color: "#00ff00".to_string(),
                text_color: "#00ffff".to_string(),
                grid_background: "#1a1a1a".to_string(),
                tile_colors: vec![
                    "#2a2a2a".to_string(), // empty
                    "#ff0080".to_string(), // 2
                    "#ff4080".to_string(), // 4
                    "#ff8080".to_string(), // 8
                    "#ffc080".to_string(), // 16
                    "#ffff80".to_string(), // 32
                    "#c0ff80".to_string(), // 64
                    "#80ff80".to_string(), // 128
                    "#80ffc0".to_string(), // 256
                    "#80ffff".to_string(), // 512
                    "#80c0ff".to_string(), // 1024
                    "#8080ff".to_string(), // 2048
                ],
            },
            "Retro" => Theme {
                background: "#2c1810".to_string(),
                title_color: "#ffd700".to_string(),
                text_color: "#ffd700".to_string(),
                grid_background: "#4a2c1a".to_string(),
                tile_colors: vec![
                    "#6a4c2a".to_string(), // empty
                    "#8a6c4a".to_string(), // 2
                    "#aa8c6a".to_string(), // 4
                    "#caac8a".to_string(), // 8
                    "#eaccaa".to_string(), // 16
                    "#ffecaa".to_string(), // 32
                    "#ffcc8a".to_string(), // 64
                    "#ffac6a".to_string(), // 128
                    "#ff8c4a".to_string(), // 256
                    "#ff6c2a".to_string(), // 512
                    "#ff4c0a".to_string(), // 1024
                    "#ff2c00".to_string(), // 2048
                ],
            },
            "Pastel" => Theme {
                background: "#f0f8ff".to_string(),
                title_color: "#87ceeb".to_string(),
                text_color: "#87ceeb".to_string(),
                grid_background: "#e6e6fa".to_string(),
                tile_colors: vec![
                    "#f5f5dc".to_string(), // empty
                    "#ffe4e1".to_string(), // 2
                    "#f0e68c".to_string(), // 4
                    "#98fb98".to_string(), // 8
                    "#87ceeb".to_string(), // 16
                    "#dda0dd".to_string(), // 32
                    "#f0e68c".to_string(), // 64
                    "#ffb6c1".to_string(), // 128
                    "#98fb98".to_string(), // 256
                    "#87ceeb".to_string(), // 512
                    "#dda0dd".to_string(), // 1024
                    "#f0e68c".to_string(), // 2048
                ],
            },
            _ => Theme { // Classic theme
                background: "#faf8ef".to_string(),
                title_color: "#776e65".to_string(),
                text_color: "#776e65".to_string(),
                grid_background: "#bbada0".to_string(),
                tile_colors: vec![
                    "#cdc1b4".to_string(), // empty
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
            },
        };
        
        serde_wasm_bindgen::to_value(&theme).unwrap()
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
