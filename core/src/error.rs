use thiserror::Error;

/// Game-related errors
#[derive(Error, Debug)]
pub enum GameError {
    #[error("Invalid move: {0}")]
    InvalidMove(String),
    
    #[error("Game is already over")]
    GameOver,
    
    #[error("Invalid board position: ({row}, {col})")]
    InvalidPosition { row: usize, col: usize },
    
    #[error("Invalid board size: {size} (must be > 0)")]
    InvalidBoardSize { size: usize },
    
    #[error("No undo available")]
    NoUndoAvailable,
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Random number generation error: {0}")]
    RngError(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

/// Result type for game operations
pub type GameResult<T> = Result<T, GameError>;
