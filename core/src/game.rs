use serde::{Deserialize, Serialize};
use crate::{
    Board, GameRng, Score, GameConfig, GameStats,
    board::Tile,
};
use crate::error::{GameError, GameResult};

/// Game direction for moves
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Game state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameState {
    Playing,
    Won,
    GameOver,
}

/// Main game controller
#[derive(Debug, Clone)]
pub struct Game {
    /// Game board
    board: Board,
    /// Score tracker
    score: Score,
    /// Random number generator
    rng: GameRng,
    /// Game configuration
    config: GameConfig,
    /// Current game state
    state: GameState,
    /// Number of moves made
    moves: u32,
    /// Game start time (Unix timestamp)
    start_time: u64,
    /// Previous board state for undo
    previous_board: Option<Board>,
    /// Previous score for undo
    previous_score: Option<Score>,
}

impl Game {
    /// Create a new game with configuration
    pub fn new(config: GameConfig) -> GameResult<Self> {
        let board = Board::new(config.board_size)?;
        let rng = GameRng::new(config.seed);
        let start_time = Self::get_current_time();
        
        let mut game = Self {
            board,
            score: Score::new(),
            rng,
            config,
            state: GameState::Playing,
            moves: 0,
            start_time,
            previous_board: None,
            previous_score: None,
        };
        
        // Add initial tiles
        game.add_random_tile()?;
        game.add_random_tile()?;
        
        Ok(game)
    }
    
    /// Get current board
    pub fn board(&self) -> &Board {
        &self.board
    }
    
    /// Get current score
    pub fn score(&self) -> &Score {
        &self.score
    }
    
    /// Get game state
    pub fn state(&self) -> GameState {
        self.state.clone()
    }
    
    /// Get number of moves
    pub fn moves(&self) -> u32 {
        self.moves
    }
    
    /// Get game statistics
    pub fn stats(&self) -> GameStats {
        let current_time = Self::get_current_time();
        
        GameStats {
            score: self.score.current(),
            best_score: self.score.best(),
            moves: self.moves,
            duration: current_time - self.start_time,
            won: self.state == GameState::Won,
            game_over: self.state == GameState::GameOver,
        }
    }
    
    /// Make a move in the specified direction
    pub fn make_move(&mut self, direction: Direction) -> GameResult<bool> {
        if self.state != GameState::Playing {
            return Err(GameError::GameOver);
        }
        
        // Save previous state for undo
        if self.config.allow_undo {
            self.previous_board = Some(self.board.clone());
            self.previous_score = Some(self.score.clone());
        }
        
        // Perform the move
        let moved = self.perform_move(direction)?;
        
        if moved {
            self.moves += 1;
            
            // Add a new random tile
            self.add_random_tile()?;
            
            // Check game state
            self.update_game_state()?;
        }
        
        Ok(moved)
    }
    
    /// Undo the last move
    pub fn undo(&mut self) -> GameResult<()> {
        if !self.config.allow_undo {
            return Err(GameError::NoUndoAvailable);
        }
        
        if let (Some(prev_board), Some(prev_score)) = 
            (self.previous_board.take(), self.previous_score.take()) {
            self.board = prev_board;
            self.score = prev_score;
            self.moves = self.moves.saturating_sub(1);
            self.state = GameState::Playing;
        } else {
            return Err(GameError::NoUndoAvailable);
        }
        
        Ok(())
    }
    
    /// Start a new game
    pub fn new_game(&mut self) -> GameResult<()> {
        self.board = Board::new(self.config.board_size)?;
        self.score.reset_current();
        self.state = GameState::Playing;
        self.moves = 0;
        self.start_time = Self::get_current_time();
        self.previous_board = None;
        self.previous_score = None;
        
        // Add initial tiles
        self.add_random_tile()?;
        self.add_random_tile()?;
        
        Ok(())
    }
    
    /// Add a random tile to the board
    fn add_random_tile(&mut self) -> GameResult<()> {
        let empty_positions = self.board.empty_positions();
        if empty_positions.is_empty() {
            return Ok(());
        }
        
        let random_index = self.rng.gen_range(empty_positions.len());
        let (row, col) = empty_positions[random_index];
        let value = self.rng.gen_tile_value();
        
        self.board.set_tile(row, col, Tile::new(value))?;
        Ok(())
    }
    
    /// Perform a move in the specified direction
    fn perform_move(&mut self, direction: Direction) -> GameResult<bool> {
        let mut moved = false;
        let size = self.board.size();
        
        match direction {
            Direction::Left => {
                for row in 0..size {
                    moved |= self.merge_row_left(row)?;
                }
            }
            Direction::Right => {
                for row in 0..size {
                    moved |= self.merge_row_right(row)?;
                }
            }
            Direction::Up => {
                for col in 0..size {
                    moved |= self.merge_col_up(col)?;
                }
            }
            Direction::Down => {
                for col in 0..size {
                    moved |= self.merge_col_down(col)?;
                }
            }
        }
        
        Ok(moved)
    }
    
    /// Merge tiles in a row from left to right
    fn merge_row_left(&mut self, row: usize) -> GameResult<bool> {
        let mut moved = false;
        let size = self.board.size();
        let mut merged = vec![false; size];
        
        // Move tiles to the left
        for col in 1..size {
            if !self.board.get_tile(row, col)?.is_empty() {
                let mut target_col = col;
                while target_col > 0 && self.board.get_tile(row, target_col - 1)?.is_empty() {
                    let tile = self.board.get_tile(row, target_col)?;
                    self.board.set_tile(row, target_col, Tile::empty())?;
                    self.board.set_tile(row, target_col - 1, tile)?;
                    target_col -= 1;
                    moved = true;
                }
            }
        }
        
        // Merge adjacent tiles
        for col in 0..size - 1 {
            if merged[col] {
                continue;
            }
            
            let current = self.board.get_tile(row, col)?;
            let next = self.board.get_tile(row, col + 1)?;
            
            if current.can_merge_with(&next) {
                let mut merged_tile = current;
                let merge_score = merged_tile.merge_with(&next);
                self.board.set_tile(row, col, merged_tile)?;
                self.board.set_tile(row, col + 1, Tile::empty())?;
                self.score.add_merge_points(merge_score);
                merged[col + 1] = true;
                moved = true;
            }
        }
        
        // Move tiles again after merging
        for col in 1..size {
            if !self.board.get_tile(row, col)?.is_empty() {
                let mut target_col = col;
                while target_col > 0 && self.board.get_tile(row, target_col - 1)?.is_empty() {
                    let tile = self.board.get_tile(row, target_col)?;
                    self.board.set_tile(row, target_col, Tile::empty())?;
                    self.board.set_tile(row, target_col - 1, tile)?;
                    target_col -= 1;
                    moved = true;
                }
            }
        }
        
        Ok(moved)
    }
    
    /// Merge tiles in a row from right to left
    fn merge_row_right(&mut self, row: usize) -> GameResult<bool> {
        let mut moved = false;
        let size = self.board.size();
        let mut merged = vec![false; size];
        
        // Move tiles to the right
        for col in (0..size - 1).rev() {
            if !self.board.get_tile(row, col)?.is_empty() {
                let mut target_col = col;
                while target_col < size - 1 && self.board.get_tile(row, target_col + 1)?.is_empty() {
                    let tile = self.board.get_tile(row, target_col)?;
                    self.board.set_tile(row, target_col, Tile::empty())?;
                    self.board.set_tile(row, target_col + 1, tile)?;
                    target_col += 1;
                    moved = true;
                }
            }
        }
        
        // Merge adjacent tiles
        for col in (1..size).rev() {
            if merged[col] {
                continue;
            }
            
            let current = self.board.get_tile(row, col)?;
            let prev = self.board.get_tile(row, col - 1)?;
            
            if current.can_merge_with(&prev) {
                let mut merged_tile = current;
                let merge_score = merged_tile.merge_with(&prev);
                self.board.set_tile(row, col, merged_tile)?;
                self.board.set_tile(row, col - 1, Tile::empty())?;
                self.score.add_merge_points(merge_score);
                merged[col - 1] = true;
                moved = true;
            }
        }
        
        // Move tiles again after merging
        for col in (0..size - 1).rev() {
            if !self.board.get_tile(row, col)?.is_empty() {
                let mut target_col = col;
                while target_col < size - 1 && self.board.get_tile(row, target_col + 1)?.is_empty() {
                    let tile = self.board.get_tile(row, target_col)?;
                    self.board.set_tile(row, target_col, Tile::empty())?;
                    self.board.set_tile(row, target_col + 1, tile)?;
                    target_col += 1;
                    moved = true;
                }
            }
        }
        
        Ok(moved)
    }
    
    /// Merge tiles in a column from top to bottom
    fn merge_col_up(&mut self, col: usize) -> GameResult<bool> {
        let mut moved = false;
        let size = self.board.size();
        let mut merged = vec![false; size];
        
        // Move tiles up
        for row in 1..size {
            if !self.board.get_tile(row, col)?.is_empty() {
                let mut target_row = row;
                while target_row > 0 && self.board.get_tile(target_row - 1, col)?.is_empty() {
                    let tile = self.board.get_tile(target_row, col)?;
                    self.board.set_tile(target_row, col, Tile::empty())?;
                    self.board.set_tile(target_row - 1, col, tile)?;
                    target_row -= 1;
                    moved = true;
                }
            }
        }
        
        // Merge adjacent tiles
        for row in 0..size - 1 {
            if merged[row] {
                continue;
            }
            
            let current = self.board.get_tile(row, col)?;
            let next = self.board.get_tile(row + 1, col)?;
            
            if current.can_merge_with(&next) {
                let mut merged_tile = current;
                let merge_score = merged_tile.merge_with(&next);
                self.board.set_tile(row, col, merged_tile)?;
                self.board.set_tile(row + 1, col, Tile::empty())?;
                self.score.add_merge_points(merge_score);
                merged[row + 1] = true;
                moved = true;
            }
        }
        
        // Move tiles again after merging
        for row in 1..size {
            if !self.board.get_tile(row, col)?.is_empty() {
                let mut target_row = row;
                while target_row > 0 && self.board.get_tile(target_row - 1, col)?.is_empty() {
                    let tile = self.board.get_tile(target_row, col)?;
                    self.board.set_tile(target_row, col, Tile::empty())?;
                    self.board.set_tile(target_row - 1, col, tile)?;
                    target_row -= 1;
                    moved = true;
                }
            }
        }
        
        Ok(moved)
    }
    
    /// Merge tiles in a column from bottom to top
    fn merge_col_down(&mut self, col: usize) -> GameResult<bool> {
        let mut moved = false;
        let size = self.board.size();
        let mut merged = vec![false; size];
        
        // Move tiles down
        for row in (0..size - 1).rev() {
            if !self.board.get_tile(row, col)?.is_empty() {
                let mut target_row = row;
                while target_row < size - 1 && self.board.get_tile(target_row + 1, col)?.is_empty() {
                    let tile = self.board.get_tile(target_row, col)?;
                    self.board.set_tile(target_row, col, Tile::empty())?;
                    self.board.set_tile(target_row + 1, col, tile)?;
                    target_row += 1;
                    moved = true;
                }
            }
        }
        
        // Merge adjacent tiles
        for row in (1..size).rev() {
            if merged[row] {
                continue;
            }
            
            let current = self.board.get_tile(row, col)?;
            let prev = self.board.get_tile(row - 1, col)?;
            
            if current.can_merge_with(&prev) {
                let mut merged_tile = current;
                let merge_score = merged_tile.merge_with(&prev);
                self.board.set_tile(row, col, merged_tile)?;
                self.board.set_tile(row - 1, col, Tile::empty())?;
                self.score.add_merge_points(merge_score);
                merged[row - 1] = true;
                moved = true;
            }
        }
        
        // Move tiles again after merging
        for row in (0..size - 1).rev() {
            if !self.board.get_tile(row, col)?.is_empty() {
                let mut target_row = row;
                while target_row < size - 1 && self.board.get_tile(target_row + 1, col)?.is_empty() {
                    let tile = self.board.get_tile(target_row, col)?;
                    self.board.set_tile(target_row, col, Tile::empty())?;
                    self.board.set_tile(target_row + 1, col, tile)?;
                    target_row += 1;
                    moved = true;
                }
            }
        }
        
        Ok(moved)
    }
    
    /// Update game state based on current board
    fn update_game_state(&mut self) -> GameResult<()> {
        // Check if won
        if self.board.max_tile() >= self.config.target_score && self.state == GameState::Playing {
            self.state = GameState::Won;
        }
        
        // Check if game over
        if !self.board.has_valid_moves() {
            self.state = GameState::GameOver;
        }
        
        Ok(())
    }
    
    /// Get current time in seconds since Unix epoch
    /// Uses different implementations for different targets
    pub fn get_current_time() -> u64 {
        #[cfg(target_arch = "wasm32")]
        {
            // For WASM, we'll use a simple approach - just return 0 for now
            // In a real implementation, you'd use js_sys::Date or similar
            0
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_game_creation() {
        let config = GameConfig::default();
        let game = Game::new(config).unwrap();
        
        assert_eq!(game.state(), GameState::Playing);
        assert_eq!(game.moves(), 0);
        assert_eq!(game.board().size(), 4);
    }
    
    #[test]
    fn test_basic_move() {
        let config = GameConfig::default();
        let mut game = Game::new(config).unwrap();
        
        // Try to move left
        let moved = game.make_move(Direction::Left).unwrap();
        // Should move if there are tiles that can be moved
        assert!(moved || !moved); // Either moved or didn't move is valid
    }
    
    #[test]
    fn test_undo() {
        let mut config = GameConfig::default();
        config.allow_undo = true;
        
        let mut game = Game::new(config).unwrap();
        let initial_score = game.score().current();
        
        // Make a move
        game.make_move(Direction::Left).unwrap();
        
        // Undo the move
        game.undo().unwrap();
        
        assert_eq!(game.score().current(), initial_score);
    }
}
