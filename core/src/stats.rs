use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::error::{GameError, GameResult};

/// Single game session statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSessionStats {
    /// Game session ID (timestamp)
    pub session_id: u64,
    /// Final score
    pub final_score: u32,
    /// Number of moves made
    pub moves: u32,
    /// Game duration in seconds
    pub duration: u64,
    /// Maximum tile achieved
    pub max_tile: u32,
    /// Whether the game was won
    pub won: bool,
    /// Game end reason
    pub end_reason: GameEndReason,
    /// Timestamp when game started
    pub start_time: u64,
    /// Timestamp when game ended
    pub end_time: u64,
    /// Average score per move
    pub avg_score_per_move: f64,
    /// Efficiency score (score / moves)
    pub efficiency: f64,
}

/// Game end reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEndReason {
    /// Game was won (reached target)
    Won,
    /// Game over (no more moves)
    GameOver,
    /// Game was abandoned
    Abandoned,
}

/// Overall statistics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsSummary {
    /// Total number of games played
    pub total_games: u32,
    /// Total number of games won
    pub games_won: u32,
    /// Win rate percentage
    pub win_rate: f64,
    /// Highest score ever achieved
    pub highest_score: u32,
    /// Average score across all games
    pub average_score: f64,
    /// Total moves across all games
    pub total_moves: u32,
    /// Average moves per game
    pub average_moves: f64,
    /// Total play time in seconds
    pub total_play_time: u64,
    /// Average game duration
    pub average_duration: f64,
    /// Highest tile ever achieved
    pub highest_tile: u32,
    /// Tile distribution (how many times each tile was achieved)
    pub tile_distribution: HashMap<u32, u32>,
    /// Score distribution (ranges)
    pub score_distribution: ScoreDistribution,
    /// Recent games (last 10)
    pub recent_games: Vec<GameSessionStats>,
}

/// Score distribution by ranges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreDistribution {
    /// Games with score 0-1000
    pub low_score: u32,
    /// Games with score 1001-5000
    pub medium_score: u32,
    /// Games with score 5001-10000
    pub high_score: u32,
    /// Games with score 10001+
    pub very_high_score: u32,
}

impl Default for ScoreDistribution {
    fn default() -> Self {
        Self {
            low_score: 0,
            medium_score: 0,
            high_score: 0,
            very_high_score: 0,
        }
    }
}

/// Statistics manager for tracking and analyzing game data
pub struct StatisticsManager {
    /// Path to statistics file
    stats_file: String,
    /// All game sessions
    sessions: Vec<GameSessionStats>,
}

impl StatisticsManager {
    /// Create a new statistics manager
    pub fn new(stats_file: &str) -> GameResult<Self> {
        let mut manager = Self {
            stats_file: stats_file.to_string(),
            sessions: Vec::new(),
        };
        
        // Load existing statistics
        manager.load_statistics()?;
        
        Ok(manager)
    }
    
    /// Record a new game session
    pub fn record_session(&mut self, session: GameSessionStats) -> GameResult<()> {
        self.sessions.push(session);
        self.save_statistics()?;
        Ok(())
    }
    
    /// Get statistics summary
    pub fn get_summary(&self) -> StatisticsSummary {
        if self.sessions.is_empty() {
            return StatisticsSummary {
                total_games: 0,
                games_won: 0,
                win_rate: 0.0,
                highest_score: 0,
                average_score: 0.0,
                total_moves: 0,
                average_moves: 0.0,
                total_play_time: 0,
                average_duration: 0.0,
                highest_tile: 0,
                tile_distribution: HashMap::new(),
                score_distribution: ScoreDistribution::default(),
                recent_games: Vec::new(),
            };
        }
        
        let total_games = self.sessions.len() as u32;
        let games_won = self.sessions.iter().filter(|s| s.won).count() as u32;
        let win_rate = (games_won as f64 / total_games as f64) * 100.0;
        
        let highest_score = self.sessions.iter().map(|s| s.final_score).max().unwrap_or(0);
        let average_score = self.sessions.iter().map(|s| s.final_score as f64).sum::<f64>() / total_games as f64;
        
        let total_moves = self.sessions.iter().map(|s| s.moves).sum::<u32>();
        let average_moves = total_moves as f64 / total_games as f64;
        
        let total_play_time = self.sessions.iter().map(|s| s.duration).sum::<u64>();
        let average_duration = total_play_time as f64 / total_games as f64;
        
        let highest_tile = self.sessions.iter().map(|s| s.max_tile).max().unwrap_or(0);
        
        // Calculate tile distribution
        let mut tile_distribution = HashMap::new();
        for session in &self.sessions {
            *tile_distribution.entry(session.max_tile).or_insert(0) += 1;
        }
        
        // Calculate score distribution
        let mut score_distribution = ScoreDistribution::default();
        for session in &self.sessions {
            match session.final_score {
                0..=1000 => score_distribution.low_score += 1,
                1001..=5000 => score_distribution.medium_score += 1,
                5001..=10000 => score_distribution.high_score += 1,
                _ => score_distribution.very_high_score += 1,
            }
        }
        
        // Get recent games (last 10)
        let mut recent_games = self.sessions.clone();
        recent_games.sort_by(|a, b| b.end_time.cmp(&a.end_time));
        recent_games.truncate(10);
        
        StatisticsSummary {
            total_games,
            games_won,
            win_rate,
            highest_score,
            average_score,
            total_moves,
            average_moves,
            total_play_time,
            average_duration,
            highest_tile,
            tile_distribution,
            score_distribution,
            recent_games,
        }
    }
    
    /// Get score trend data (last N games)
    pub fn get_score_trend(&self, count: usize) -> Vec<(u32, u32)> {
        let mut recent_sessions = self.sessions.clone();
        recent_sessions.sort_by(|a, b| b.end_time.cmp(&a.end_time));
        recent_sessions.truncate(count);
        recent_sessions.reverse();
        
        recent_sessions
            .iter()
            .enumerate()
            .map(|(i, session)| (i as u32, session.final_score))
            .collect()
    }
    
    /// Get efficiency trend data (last N games)
    pub fn get_efficiency_trend(&self, count: usize) -> Vec<(u32, f64)> {
        let mut recent_sessions = self.sessions.clone();
        recent_sessions.sort_by(|a, b| b.end_time.cmp(&a.end_time));
        recent_sessions.truncate(count);
        recent_sessions.reverse();
        
        recent_sessions
            .iter()
            .enumerate()
            .map(|(i, session)| (i as u32, session.efficiency))
            .collect()
    }
    
    /// Get tile achievement data
    pub fn get_tile_achievements(&self) -> Vec<(u32, u32)> {
        let mut tile_counts: Vec<(u32, u32)> = self.sessions
            .iter()
            .fold(HashMap::new(), |mut acc, session| {
                *acc.entry(session.max_tile).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .collect();
        
        tile_counts.sort_by(|a, b| a.0.cmp(&b.0));
        tile_counts
    }
    
    /// Load statistics from file
    fn load_statistics(&mut self) -> GameResult<()> {
        if !Path::new(&self.stats_file).exists() {
            return Ok(());
        }
        
        let content = fs::read_to_string(&self.stats_file)
            .map_err(|e| GameError::InvalidOperation(format!("Failed to read stats file: {}", e)))?;
        
        self.sessions = serde_json::from_str(&content)
            .map_err(|e| GameError::InvalidOperation(format!("Failed to parse stats file: {}", e)))?;
        
        Ok(())
    }
    
    /// Save statistics to file
    fn save_statistics(&self) -> GameResult<()> {
        let content = serde_json::to_string_pretty(&self.sessions)
            .map_err(|e| GameError::InvalidOperation(format!("Failed to serialize stats: {}", e)))?;
        
        fs::write(&self.stats_file, content)
            .map_err(|e| GameError::InvalidOperation(format!("Failed to write stats file: {}", e)))?;
        
        Ok(())
    }
    
    /// Clear all statistics
    pub fn clear_statistics(&mut self) -> GameResult<()> {
        self.sessions.clear();
        self.save_statistics()?;
        Ok(())
    }
    
    /// Export statistics to JSON
    pub fn export_statistics(&self) -> GameResult<String> {
        serde_json::to_string_pretty(&self.sessions)
            .map_err(|e| GameError::InvalidOperation(format!("Failed to export stats: {}", e)))
    }
}

/// Helper function to create a game session from game stats
pub fn create_session_stats(
    final_score: u32,
    moves: u32,
    duration: u64,
    max_tile: u32,
    won: bool,
    start_time: u64,
    end_time: u64,
) -> GameSessionStats {
    let end_reason = if won {
        GameEndReason::Won
    } else {
        GameEndReason::GameOver
    };
    
    let avg_score_per_move = if moves > 0 {
        final_score as f64 / moves as f64
    } else {
        0.0
    };
    
    let efficiency = if moves > 0 {
        final_score as f64 / moves as f64
    } else {
        0.0
    };
    
    GameSessionStats {
        session_id: start_time,
        final_score,
        moves,
        duration,
        max_tile,
        won,
        end_reason,
        start_time,
        end_time,
        avg_score_per_move,
        efficiency,
    }
}
