use serde::{Deserialize, Serialize};
use crate::{Direction, Game, GameConfig, GameResult, GameError};

/// A single move in the replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayMove {
    /// Direction of the move
    pub direction: Direction,
    /// Board state before the move
    pub board_before: Vec<Vec<u32>>,
    /// Board state after the move
    pub board_after: Vec<Vec<u32>>,
    /// Score before the move
    pub score_before: u32,
    /// Score after the move
    pub score_after: u32,
    /// Move number
    pub move_number: u32,
    /// Timestamp of the move
    pub timestamp: u64,
}

/// Complete replay data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayData {
    /// Game configuration
    pub config: GameConfig,
    /// Initial board state
    pub initial_board: Vec<Vec<u32>>,
    /// All moves in the replay
    pub moves: Vec<ReplayMove>,
    /// Final game state
    pub final_state: crate::GameState,
    /// Final score
    pub final_score: u32,
    /// Total moves
    pub total_moves: u32,
    /// Game duration
    pub duration: u64,
    /// Replay metadata
    pub metadata: ReplayMetadata,
}

/// Replay metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayMetadata {
    /// Replay name/title
    pub name: String,
    /// Creation timestamp
    pub created_at: u64,
    /// Player name (optional)
    pub player_name: Option<String>,
    /// Game version
    pub version: String,
    /// Additional notes
    pub notes: Option<String>,
}

impl Default for ReplayMetadata {
    fn default() -> Self {
        Self {
            name: "Untitled Replay".to_string(),
            created_at: crate::game::Game::get_current_time(),
            player_name: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
            notes: None,
        }
    }
}

impl ReplayMetadata {
    /// Create a new replay metadata with custom name
    pub fn new(name: String) -> Self {
        Self {
            name,
            created_at: crate::game::Game::get_current_time(),
            player_name: None,
            version: env!("CARGO_PKG_VERSION").to_string(),
            notes: None,
        }
    }
    
    /// Set player name
    pub fn with_player_name(mut self, player_name: String) -> Self {
        self.player_name = Some(player_name);
        self
    }
    
    /// Set notes
    pub fn with_notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }
}

/// Replay recorder for capturing game moves
pub struct ReplayRecorder {
    /// Game being recorded
    game: Game,
    /// Replay data being built
    replay_data: ReplayData,
    /// Whether recording is active
    recording: bool,
}

impl ReplayRecorder {
    /// Create a new replay recorder
    pub fn new(config: GameConfig) -> GameResult<Self> {
        let game = Game::new(config.clone())?;
        let initial_board = game.board().to_vec();
        
        let replay_data = ReplayData {
            config,
            initial_board,
            moves: Vec::new(),
            final_state: game.state(),
            final_score: game.score().current(),
            total_moves: 0,
            duration: 0,
            metadata: ReplayMetadata::default(),
        };
        
        Ok(Self {
            game,
            replay_data,
            recording: true,
        })
    }
    
    /// Make a move and record it
    pub fn make_move(&mut self, direction: Direction) -> GameResult<bool> {
        if !self.recording {
            return Err(GameError::InvalidOperation("Recording stopped".to_string()));
        }
        
        // Save state before move
        let board_before = self.game.board().to_vec();
        let score_before = self.game.score().current();
        let move_number = self.game.moves();
        let timestamp = crate::game::Game::get_current_time();
        
        // Make the move
        let moved = self.game.make_move(direction)?;
        
        if moved {
            // Record the move
            let move_record = ReplayMove {
                direction,
                board_before,
                board_after: self.game.board().to_vec(),
                score_before,
                score_after: self.game.score().current(),
                move_number,
                timestamp,
            };
            
            self.replay_data.moves.push(move_record);
            self.replay_data.total_moves = self.game.moves();
            self.replay_data.final_state = self.game.state();
            self.replay_data.final_score = self.game.score().current();
        }
        
        Ok(moved)
    }
    
    /// Stop recording and finalize replay
    pub fn stop_recording(&mut self) -> ReplayData {
        self.recording = false;
        self.replay_data.duration = crate::game::Game::get_current_time() - self.replay_data.metadata.created_at;
        self.replay_data.clone()
    }
    
    /// Get current game state
    pub fn game(&self) -> &Game {
        &self.game
    }
    
    /// Get current replay data
    pub fn replay_data(&self) -> &ReplayData {
        &self.replay_data
    }
    
    /// Set replay metadata
    pub fn set_metadata(&mut self, metadata: ReplayMetadata) {
        self.replay_data.metadata = metadata;
    }
}

/// Replay player for playing back recorded games
pub struct ReplayPlayer {
    /// Replay data to play
    replay_data: ReplayData,
    /// Current position in the replay
    current_move: usize,
    /// Current game state
    current_game: Game,
    /// Whether replay is playing
    playing: bool,
    /// Playback speed (1.0 = normal, 2.0 = 2x speed, etc.)
    speed: f32,
}

impl ReplayPlayer {
    /// Create a new replay player
    pub fn new(replay_data: ReplayData) -> GameResult<Self> {
        let current_game = Game::new(replay_data.config.clone())?;
        
        Ok(Self {
            replay_data,
            current_move: 0,
            current_game,
            playing: false,
            speed: 1.0,
        })
    }
    
    /// Start playing the replay
    pub fn play(&mut self) {
        self.playing = true;
    }
    
    /// Pause the replay
    pub fn pause(&mut self) {
        self.playing = false;
    }
    
    /// Stop the replay and reset to beginning
    pub fn stop(&mut self) {
        self.playing = false;
        self.current_move = 0;
        self.reset_game();
    }
    
    /// Go to next move
    pub fn next_move(&mut self) -> GameResult<bool> {
        if self.current_move >= self.replay_data.moves.len() {
            return Ok(false);
        }
        
        let replay_move = &self.replay_data.moves[self.current_move];
        
        // Apply the move to current game
        self.current_game.make_move(replay_move.direction)?;
        self.current_move += 1;
        
        Ok(true)
    }
    
    /// Go to previous move
    pub fn previous_move(&mut self) -> GameResult<bool> {
        if self.current_move == 0 {
            return Ok(false);
        }
        
        self.current_move -= 1;
        self.reset_to_move(self.current_move)?;
        
        Ok(true)
    }
    
    /// Go to specific move
    pub fn go_to_move(&mut self, move_index: usize) -> GameResult<bool> {
        if move_index > self.replay_data.moves.len() {
            return Err(GameError::InvalidOperation("Move index out of bounds".to_string()));
        }
        
        self.current_move = move_index;
        self.reset_to_move(move_index)?;
        
        Ok(true)
    }
    
    /// Reset game to initial state
    fn reset_game(&mut self) {
        self.current_game = Game::new(self.replay_data.config.clone()).unwrap();
    }
    
    /// Reset game to specific move
    fn reset_to_move(&mut self, move_index: usize) -> GameResult<()> {
        self.reset_game();
        
        for i in 0..move_index {
            let replay_move = &self.replay_data.moves[i];
            self.current_game.make_move(replay_move.direction)?;
        }
        
        Ok(())
    }
    
    /// Get current game state
    pub fn current_game(&self) -> &Game {
        &self.current_game
    }
    
    /// Get current move index
    pub fn current_move_index(&self) -> usize {
        self.current_move
    }
    
    /// Get total moves
    pub fn total_moves(&self) -> usize {
        self.replay_data.moves.len()
    }
    
    /// Get replay data
    pub fn replay_data(&self) -> &ReplayData {
        &self.replay_data
    }
    
    /// Check if replay is finished
    pub fn is_finished(&self) -> bool {
        self.current_move >= self.replay_data.moves.len()
    }
    
    /// Check if replay is playing
    pub fn is_playing(&self) -> bool {
        self.playing
    }
    
    /// Set playback speed
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.max(0.1).min(10.0);
    }
    
    /// Get playback speed
    pub fn speed(&self) -> f32 {
        self.speed
    }
    
    /// Get progress percentage
    pub fn progress(&self) -> f32 {
        if self.replay_data.moves.is_empty() {
            0.0
        } else {
            (self.current_move as f32 / self.replay_data.moves.len() as f32) * 100.0
        }
    }
}

/// Replay manager for handling multiple replays
pub struct ReplayManager {
    /// List of saved replays
    replays: Vec<ReplayData>,
}

impl ReplayManager {
    /// Create a new replay manager
    pub fn new() -> Self {
        Self {
            replays: Vec::new(),
        }
    }
    
    /// Add a replay to the manager
    pub fn add_replay(&mut self, replay: ReplayData) {
        self.replays.push(replay);
    }
    
    /// Get all replays
    pub fn get_replays(&self) -> &[ReplayData] {
        &self.replays
    }
    
    /// Get replay by index
    pub fn get_replay(&self, index: usize) -> Option<&ReplayData> {
        self.replays.get(index)
    }
    
    /// Remove replay by index
    pub fn remove_replay(&mut self, index: usize) -> Option<ReplayData> {
        if index < self.replays.len() {
            Some(self.replays.remove(index))
        } else {
            None
        }
    }
    
    /// Clear all replays
    pub fn clear_replays(&mut self) {
        self.replays.clear();
    }
    
    /// Get number of replays
    pub fn replay_count(&self) -> usize {
        self.replays.len()
    }
}

impl Default for ReplayManager {
    fn default() -> Self {
        Self::new()
    }
}
