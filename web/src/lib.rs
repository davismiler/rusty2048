use wasm_bindgen::prelude::*;
use rusty2048_core::{Game, GameConfig, Direction};
use rusty2048_shared::Theme;
use console_error_panic_hook;
use web_sys::console;

#[wasm_bindgen]
pub struct Rusty2048Web {
    game: Game,
    theme: Theme,
}

#[wasm_bindgen]
impl Rusty2048Web {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Rusty2048Web, JsValue> {
        // Set up panic hook for better error messages
        console_error_panic_hook::set_once();
        
        console::log_1(&"Initializing Rusty2048Web...".into());
        
        let mut config = GameConfig::default();
        // Use a fixed seed for WASM to avoid entropy issues
        config.seed = Some(42);
        
        console::log_1(&"Creating game with config...".into());
        let game = Game::new(config).map_err(|e| {
            let error_msg = format!("Failed to create game: {}", e);
            console::error_1(&error_msg.into());
            JsValue::from_str(&e.to_string())
        })?;
        
        console::log_1(&"Game created successfully!".into());
        let theme = Theme::default();
        Ok(Rusty2048Web { game, theme })
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
    
    pub fn get_board(&self) -> Result<JsValue, JsValue> {
        let board = self.game.board();
        let size = board.size();
        let mut tiles = Vec::new();
        
        for row in 0..size {
            for col in 0..size {
                let tile = board.get_tile(row, col)
                    .map_err(|e| JsValue::from_str(&e.to_string()))?;
                tiles.push(tile.value);
            }
        }
        
        serde_wasm_bindgen::to_value(&tiles)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    
    pub fn get_score(&self) -> Result<JsValue, JsValue> {
        let score = self.game.score();
        let score_data = serde_json::json!({
            "current": score.current(),
            "best": score.best(),
            "last_move": score.last_move()
        });
        
        serde_wasm_bindgen::to_value(&score_data)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    
    pub fn get_state(&self) -> Result<String, JsValue> {
        match self.game.state() {
            rusty2048_core::GameState::Playing => Ok("playing".to_string()),
            rusty2048_core::GameState::Won => Ok("won".to_string()),
            rusty2048_core::GameState::GameOver => Ok("game_over".to_string()),
        }
    }
    
    pub fn new_game(&mut self) -> Result<(), JsValue> {
        self.game.new_game().map_err(|e| JsValue::from_str(&e.to_string()))
    }
    
    pub fn undo(&mut self) -> Result<(), JsValue> {
        self.game.undo().map_err(|e| JsValue::from_str(&e.to_string()))
    }
    
    pub fn get_moves(&self) -> u32 {
        self.game.moves()
    }
    
    pub fn get_stats(&self) -> Result<JsValue, JsValue> {
        let stats = self.game.stats();
        let stats_data = serde_json::json!({
            "duration": stats.duration,
            "max_tile": self.game.board().max_tile(),
            "moves": self.game.moves(),
            "score": self.game.score().current(),
            "best_score": self.game.score().best()
        });
        
        serde_wasm_bindgen::to_value(&stats_data)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    
    pub fn get_theme(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.theme)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    
    pub fn set_theme(&mut self, theme_name: &str) -> Result<(), JsValue> {
        if let Some(theme) = Theme::by_name(theme_name) {
            self.theme = theme;
            Ok(())
        } else {
            Err(JsValue::from_str("Invalid theme name"))
        }
    }
    
    pub fn get_available_themes(&self) -> Result<JsValue, JsValue> {
        let themes = Theme::all_themes();
        let theme_names: Vec<String> = themes.iter().map(|t| t.name.clone()).collect();
        serde_wasm_bindgen::to_value(&theme_names)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
    
    pub fn get_max_tile(&self) -> u32 {
        self.game.board().max_tile()
    }
}

/// Initialize panic hook for better error messages
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
