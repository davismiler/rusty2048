use serde::{Deserialize, Serialize};
use crate::error::{GameError, GameResult};

/// Represents a single tile on the game board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tile {
    /// Tile value (0 for empty, 2^n for filled tiles)
    pub value: u32,
}

impl Tile {
    /// Create a new empty tile
    pub fn empty() -> Self {
        Self { value: 0 }
    }
    
    /// Create a new tile with a specific value
    pub fn new(value: u32) -> Self {
        Self { value }
    }
    
    /// Check if the tile is empty
    pub fn is_empty(&self) -> bool {
        self.value == 0
    }
    
    /// Check if the tile can merge with another tile
    pub fn can_merge_with(&self, other: &Tile) -> bool {
        !self.is_empty() && !other.is_empty() && self.value == other.value
    }
    
    /// Merge this tile with another tile
    pub fn merge_with(&mut self, other: &Tile) -> u32 {
        if self.can_merge_with(other) {
            self.value *= 2;
            self.value
        } else {
            0
        }
    }
    
    /// Get the color index for UI rendering
    pub fn color_index(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.value.trailing_zeros() as usize
        }
    }
}

/// Game board representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    /// 2D grid of tiles
    tiles: Vec<Vec<Tile>>,
    /// Board size (width = height)
    size: usize,
}

impl Board {
    /// Create a new empty board
    pub fn new(size: usize) -> GameResult<Self> {
        if size == 0 {
            return Err(GameError::InvalidBoardSize { size });
        }
        
        let tiles = vec![vec![Tile::empty(); size]; size];
        Ok(Self { tiles, size })
    }
    
    /// Get board size
    pub fn size(&self) -> usize {
        self.size
    }
    
    /// Get tile at position
    pub fn get_tile(&self, row: usize, col: usize) -> GameResult<Tile> {
        if row >= self.size || col >= self.size {
            return Err(GameError::InvalidPosition { row, col });
        }
        Ok(self.tiles[row][col])
    }
    
    /// Set tile at position
    pub fn set_tile(&mut self, row: usize, col: usize, tile: Tile) -> GameResult<()> {
        if row >= self.size || col >= self.size {
            return Err(GameError::InvalidPosition { row, col });
        }
        self.tiles[row][col] = tile;
        Ok(())
    }
    
    /// Check if position is empty
    pub fn is_empty(&self, row: usize, col: usize) -> GameResult<bool> {
        Ok(self.get_tile(row, col)?.is_empty())
    }
    
    /// Get all empty positions
    pub fn empty_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for row in 0..self.size {
            for col in 0..self.size {
                if self.tiles[row][col].is_empty() {
                    positions.push((row, col));
                }
            }
        }
        positions
    }
    
    /// Check if board is full
    pub fn is_full(&self) -> bool {
        self.empty_positions().is_empty()
    }
    
    /// Check if any moves are possible
    pub fn has_valid_moves(&self) -> bool {
        // Check for empty tiles
        if !self.is_full() {
            return true;
        }
        
        // Check for possible merges
        for row in 0..self.size {
            for col in 0..self.size {
                let current = self.tiles[row][col];
                
                // Check right neighbor
                if col + 1 < self.size && current.can_merge_with(&self.tiles[row][col + 1]) {
                    return true;
                }
                
                // Check bottom neighbor
                if row + 1 < self.size && current.can_merge_with(&self.tiles[row + 1][col]) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Get a copy of the current board state
    pub fn clone(&self) -> Self {
        Self {
            tiles: self.tiles.clone(),
            size: self.size,
        }
    }
    
    /// Get the maximum tile value on the board
    pub fn max_tile(&self) -> u32 {
        self.tiles
            .iter()
            .flat_map(|row| row.iter())
            .map(|tile| tile.value)
            .max()
            .unwrap_or(0)
    }
    
    /// Count tiles with a specific value
    pub fn count_tiles(&self, value: u32) -> usize {
        self.tiles
            .iter()
            .flat_map(|row| row.iter())
            .filter(|tile| tile.value == value)
            .count()
    }
    
    /// Convert board to 2D vector of u32 values
    pub fn to_vec(&self) -> Vec<Vec<u32>> {
        self.tiles.iter()
            .map(|row| row.iter().map(|tile| tile.value).collect())
            .collect()
    }
    
    /// Create board from 2D vector of u32 values
    pub fn from_vec(values: Vec<Vec<u32>>) -> GameResult<Self> {
        if values.is_empty() || values[0].is_empty() {
            return Err(GameError::InvalidBoardSize { size: 0 });
        }
        
        let size = values.len();
        if values.iter().any(|row| row.len() != size) {
            return Err(GameError::InvalidBoardSize { size });
        }
        
        let tiles = values.iter()
            .map(|row| row.iter().map(|&value| Tile::new(value)).collect())
            .collect();
        
        Ok(Self { tiles, size })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_board_creation() {
        let board = Board::new(4).unwrap();
        assert_eq!(board.size(), 4);
        assert!(board.is_empty(0, 0).unwrap());
    }
    
    #[test]
    fn test_invalid_board_size() {
        assert!(Board::new(0).is_err());
    }
    
    #[test]
    fn test_tile_operations() {
        let mut tile = Tile::empty();
        assert!(tile.is_empty());
        
        tile = Tile::new(2);
        assert!(!tile.is_empty());
        assert_eq!(tile.value, 2);
        
        let other = Tile::new(2);
        assert!(tile.can_merge_with(&other));
        assert_eq!(tile.merge_with(&other), 4);
    }
    
    #[test]
    fn test_board_operations() {
        let mut board = Board::new(4).unwrap();
        
        // Test setting and getting tiles
        board.set_tile(0, 0, Tile::new(2)).unwrap();
        assert_eq!(board.get_tile(0, 0).unwrap().value, 2);
        
        // Test invalid positions
        assert!(board.get_tile(4, 0).is_err());
        assert!(board.set_tile(0, 4, Tile::new(2)).is_err());
        
        // Test empty positions
        let empty = board.empty_positions();
        assert_eq!(empty.len(), 15); // 16 - 1 = 15 empty positions
        
        // Test max tile
        assert_eq!(board.max_tile(), 2);
    }
}
