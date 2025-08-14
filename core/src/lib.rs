//! Core game logic for Rusty2048
//! 
//! This module provides the fundamental game mechanics including:
//! - Game board representation
//! - Move validation and execution
//! - Score calculation
//! - Game state management
//! - Random number generation with seed support

pub mod board;
pub mod game;
pub mod rng;
pub mod score;
pub mod error;
pub mod replay;

pub use board::Board;
pub use game::{Game, GameState, Direction};
pub use rng::GameRng;
pub use score::Score;
pub use error::{GameError, GameResult};
pub use replay::{ReplayRecorder, ReplayPlayer, ReplayManager, ReplayData, ReplayMove, ReplayMetadata};

/// Game configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameConfig {
    /// Board size (default: 4)
    pub board_size: usize,
    /// Target score to win (default: 2048)
    pub target_score: u32,
    /// Whether to allow undo (default: true)
    pub allow_undo: bool,
    /// Random seed for reproducible games
    pub seed: Option<u64>,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            board_size: 4,
            target_score: 2048,
            allow_undo: true,
            seed: None,
        }
    }
}

/// Game statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameStats {
    /// Current score
    pub score: u32,
    /// Best score achieved
    pub best_score: u32,
    /// Number of moves made
    pub moves: u32,
    /// Game duration in seconds
    pub duration: u64,
    /// Whether the game is won
    pub won: bool,
    /// Whether the game is over
    pub game_over: bool,
}

impl Default for GameStats {
    fn default() -> Self {
        Self {
            score: 0,
            best_score: 0,
            moves: 0,
            duration: 0,
            won: false,
            game_over: false,
        }
    }
}
