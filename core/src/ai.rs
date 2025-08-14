use crate::{Board, Direction, Game, GameConfig, GameResult};
use crate::board::Tile;

/// AI algorithm types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIAlgorithm {
    /// Simple greedy algorithm
    Greedy,
    /// Expectimax algorithm with limited depth
    Expectimax,
    /// Monte Carlo Tree Search
    MCTS,
}

/// AI player for 2048 game
pub struct AIPlayer {
    algorithm: AIAlgorithm,
    max_depth: usize,
    simulation_count: usize,
}

impl AIPlayer {
    /// Create a new AI player
    pub fn new(algorithm: AIAlgorithm) -> Self {
        let max_depth = match algorithm {
            AIAlgorithm::Greedy => 1,
            AIAlgorithm::Expectimax => 4,
            AIAlgorithm::MCTS => 1000,
        };
        
        let simulation_count = match algorithm {
            AIAlgorithm::Greedy => 1,
            AIAlgorithm::Expectimax => 1,
            AIAlgorithm::MCTS => 100,
        };
        
        Self {
            algorithm,
            max_depth,
            simulation_count,
        }
    }
    
    /// Set the maximum search depth
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }
    
    /// Set the number of simulations for MCTS
    pub fn with_simulation_count(mut self, count: usize) -> Self {
        self.simulation_count = count;
        self
    }
    
    /// Get the best move for the current game state
    pub fn get_best_move(&self, game: &Game) -> GameResult<Direction> {
        match self.algorithm {
            AIAlgorithm::Greedy => self.greedy_move(game),
            AIAlgorithm::Expectimax => self.expectimax_move(game),
            AIAlgorithm::MCTS => self.mcts_move(game),
        }
    }
    
    /// Simple greedy algorithm - choose the move that gives the highest immediate score
    fn greedy_move(&self, game: &Game) -> GameResult<Direction> {
        let mut best_score = 0;
        let mut best_direction = Direction::Up;
        
        for &direction in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            let mut game_copy = game.clone();
            if let Ok(moved) = game_copy.make_move(direction) {
                if moved {
                    let score = game_copy.score().current();
                    if score > best_score {
                        best_score = score;
                        best_direction = direction;
                    }
                }
            }
        }
        
        Ok(best_direction)
    }
    
    /// Expectimax algorithm - considers both player moves and random tile placements
    fn expectimax_move(&self, game: &Game) -> GameResult<Direction> {
        let mut best_score = f64::NEG_INFINITY;
        let mut best_direction = Direction::Up;
        
        for &direction in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            let mut game_copy = game.clone();
            if let Ok(moved) = game_copy.make_move(direction) {
                if moved {
                    let score = self.expectimax_search(&game_copy, self.max_depth - 1, false);
                    if score > best_score {
                        best_score = score;
                        best_direction = direction;
                    }
                }
            }
        }
        
        Ok(best_direction)
    }
    
    /// Expectimax search implementation
    fn expectimax_search(&self, game: &Game, depth: usize, is_maximizing: bool) -> f64 {
        if depth == 0 || game.state() != crate::GameState::Playing {
            return self.evaluate_board(game.board());
        }
        
        if is_maximizing {
            // Player's turn - maximize score
            let mut max_score = f64::NEG_INFINITY;
            for &direction in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                let mut game_copy = game.clone();
                if let Ok(moved) = game_copy.make_move(direction) {
                    if moved {
                        let score = self.expectimax_search(&game_copy, depth - 1, false);
                        max_score = max_score.max(score);
                    }
                }
            }
            max_score
        } else {
            // Random tile placement - expect average score
            let empty_positions = game.board().empty_positions();
            if empty_positions.is_empty() {
                return self.evaluate_board(game.board());
            }
            
            let mut total_score = 0.0;
            let mut count = 0;
            
            // Sample a few random tile placements
            for _ in 0..self.simulation_count.min(empty_positions.len()) {
                let mut game_copy = game.clone();
                if let Ok(()) = self.add_random_tile_simulation(&mut game_copy) {
                    let score = self.expectimax_search(&game_copy, depth - 1, true);
                    total_score += score;
                    count += 1;
                }
            }
            
            if count > 0 {
                total_score / count as f64
            } else {
                self.evaluate_board(game.board())
            }
        }
    }
    
    /// Monte Carlo Tree Search algorithm
    fn mcts_move(&self, game: &Game) -> GameResult<Direction> {
        let mut root = MCTSNode::new(game.clone());
        
        for _ in 0..self.simulation_count {
            let mut current = &mut root;
            let mut game_state = game.clone();
            
            // Selection
            while current.children.len() > 0 && current.visits > 0 {
                current = current.select_child();
                if let Some(direction) = current.last_move {
                    let _ = game_state.make_move(direction);
                }
            }
            
            // Expansion
            if current.visits > 0 && game_state.state() == crate::GameState::Playing {
                current.expand(&game_state);
            }
            
            // Simulation
            let mut simulation_game = game_state.clone();
            let simulation_result = self.simulate_random_game(&mut simulation_game);
            
            // Backpropagation
            current.backpropagate(simulation_result);
        }
        
        // Choose the best move
        let best_child = root.children.iter()
            .max_by(|a, b| a.visits.cmp(&b.visits))
            .ok_or_else(|| crate::GameError::InvalidOperation("No valid moves".to_string()))?;
        
        Ok(best_child.last_move.unwrap_or(Direction::Up))
    }
    
    /// Simulate a random game to completion
    fn simulate_random_game(&self, game: &mut Game) -> f64 {
        let mut moves = 0;
        let max_moves = 1000; // Prevent infinite loops
        
        while game.state() == crate::GameState::Playing && moves < max_moves {
            let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];
            let mut moved = false;
            
            for &direction in &directions {
                if let Ok(did_move) = game.make_move(direction) {
                    if did_move {
                        moved = true;
                        break;
                    }
                }
            }
            
            if !moved {
                break; // No valid moves
            }
            
            moves += 1;
        }
        
        self.evaluate_board(game.board())
    }
    
    /// Add a random tile for simulation purposes
    fn add_random_tile_simulation(&self, game: &mut Game) -> GameResult<()> {
        let empty_positions = game.board().empty_positions();
        if empty_positions.is_empty() {
            return Ok(());
        }
        
        // Use a simple random selection for simulation
        let random_index = (empty_positions.len() as f64 * 0.5) as usize; // Simplified
        let (row, col) = empty_positions[random_index];
        let value = if rand::random::<u64>() % 10 < 9 { 2 } else { 4 };
        
        game.board_mut().set_tile(row, col, Tile::new(value))?;
        Ok(())
    }
    
    /// Evaluate the current board state
    fn evaluate_board(&self, board: &Board) -> f64 {
        let mut score = 0.0;
        let size = board.size();
        
        // Weight matrix for position importance (corner and edge tiles are more valuable)
        let weights = vec![
            vec![4.0, 2.0, 2.0, 4.0],
            vec![2.0, 1.0, 1.0, 2.0],
            vec![2.0, 1.0, 1.0, 2.0],
            vec![4.0, 2.0, 2.0, 4.0],
        ];
        
        // Evaluate each tile
        for row in 0..size {
            for col in 0..size {
                if let Ok(tile) = board.get_tile(row, col) {
                    if !tile.is_empty() {
                        let weight = if row < weights.len() && col < weights[row].len() {
                            weights[row][col]
                        } else {
                            1.0
                        };
                        score += (tile.value as f64) * weight;
                    }
                }
            }
        }
        
        // Bonus for keeping high values in corners
        score += self.corner_bonus(board);
        
        // Penalty for having many small tiles scattered
        score -= self.scattered_penalty(board);
        
        // Bonus for smoothness (adjacent tiles with similar values)
        score += self.smoothness_bonus(board);
        
        score
    }
    
    /// Bonus for keeping high values in corners
    fn corner_bonus(&self, board: &Board) -> f64 {
        let size = board.size();
        let mut bonus = 0.0;
        
        let corners = [(0, 0), (0, size-1), (size-1, 0), (size-1, size-1)];
        
        for (row, col) in corners {
            if let Ok(tile) = board.get_tile(row, col) {
                if !tile.is_empty() {
                    bonus += tile.value as f64 * 2.0;
                }
            }
        }
        
        bonus
    }
    
    /// Penalty for having many small tiles scattered
    fn scattered_penalty(&self, board: &Board) -> f64 {
        let size = board.size();
        let mut penalty = 0.0;
        let mut small_tiles = 0;
        
        for row in 0..size {
            for col in 0..size {
                if let Ok(tile) = board.get_tile(row, col) {
                    if !tile.is_empty() && tile.value <= 8 {
                        small_tiles += 1;
                    }
                }
            }
        }
        
        penalty += small_tiles as f64 * 0.5;
        penalty
    }
    
    /// Bonus for smoothness (adjacent tiles with similar values)
    fn smoothness_bonus(&self, board: &Board) -> f64 {
        let size = board.size();
        let mut bonus = 0.0;
        
        // Check horizontal smoothness
        for row in 0..size {
            for col in 0..size-1 {
                if let (Ok(tile1), Ok(tile2)) = (board.get_tile(row, col), board.get_tile(row, col+1)) {
                    if !tile1.is_empty() && !tile2.is_empty() {
                        let diff = (tile1.value as f64 - tile2.value as f64).abs();
                        bonus -= diff * 0.1;
                    }
                }
            }
        }
        
        // Check vertical smoothness
        for row in 0..size-1 {
            for col in 0..size {
                if let (Ok(tile1), Ok(tile2)) = (board.get_tile(row, col), board.get_tile(row+1, col)) {
                    if !tile1.is_empty() && !tile2.is_empty() {
                        let diff = (tile1.value as f64 - tile2.value as f64).abs();
                        bonus -= diff * 0.1;
                    }
                }
            }
        }
        
        bonus
    }
}

/// MCTS Node for Monte Carlo Tree Search
struct MCTSNode {
    game: Game,
    children: Vec<MCTSNode>,
    visits: usize,
    total_score: f64,
    last_move: Option<Direction>,
}

impl MCTSNode {
    fn new(game: Game) -> Self {
        Self {
            game,
            children: Vec::new(),
            visits: 0,
            total_score: 0.0,
            last_move: None,
        }
    }
    
    fn select_child(&mut self) -> &mut MCTSNode {
        // UCB1 formula for selection
        let c = 1.414; // Exploration constant
        let log_parent_visits = (self.visits as f64).ln();
        
        if self.children.is_empty() {
            panic!("Cannot select child from empty children list");
        }
        
        let mut best_index = 0;
        let mut best_ucb = f64::NEG_INFINITY;
        
        for (i, child) in self.children.iter().enumerate() {
            let ucb = child.total_score / child.visits as f64 + c * (log_parent_visits / child.visits as f64).sqrt();
            if ucb > best_ucb {
                best_ucb = ucb;
                best_index = i;
            }
        }
        
        &mut self.children[best_index]
    }
    
    fn expand(&mut self, game: &Game) {
        for &direction in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            let mut game_copy = game.clone();
            if let Ok(moved) = game_copy.make_move(direction) {
                if moved {
                    let mut child = MCTSNode::new(game_copy);
                    child.last_move = Some(direction);
                    self.children.push(child);
                }
            }
        }
    }
    
    fn backpropagate(&mut self, score: f64) {
        self.visits += 1;
        self.total_score += score;
    }
}

/// AI Game Controller - manages AI gameplay
pub struct AIGameController {
    ai_player: AIPlayer,
    game: Game,
    auto_play: bool,
    move_delay_ms: u64,
}

impl AIGameController {
    /// Create a new AI game controller
    pub fn new(config: GameConfig, algorithm: AIAlgorithm) -> GameResult<Self> {
        let ai_player = AIPlayer::new(algorithm);
        let game = Game::new(config)?;
        
        Ok(Self {
            ai_player,
            game,
            auto_play: false,
            move_delay_ms: 500,
        })
    }
    
    /// Set auto-play mode
    pub fn set_auto_play(&mut self, auto_play: bool) {
        self.auto_play = auto_play;
    }
    
    /// Set move delay for auto-play
    pub fn set_move_delay(&mut self, delay_ms: u64) {
        self.move_delay_ms = delay_ms;
    }
    
    /// Get the current game
    pub fn game(&self) -> &Game {
        &self.game
    }
    
    /// Get a mutable reference to the game
    pub fn game_mut(&mut self) -> &mut Game {
        &mut self.game
    }
    
    /// Make an AI move
    pub fn make_ai_move(&mut self) -> GameResult<bool> {
        if self.game.state() != crate::GameState::Playing {
            return Ok(false);
        }
        
        let best_move = self.ai_player.get_best_move(&self.game)?;
        self.game.make_move(best_move)
    }
    
    /// Start a new AI game
    pub fn new_game(&mut self) -> GameResult<()> {
        self.game.new_game()
    }
    
    /// Get the AI algorithm being used
    pub fn algorithm(&self) -> AIAlgorithm {
        self.ai_player.algorithm
    }
}

// Add rand dependency for simulation
mod rand {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;
    
    pub fn random<T>() -> T 
    where 
        T: Copy + From<u64>,
    {
        let mut hasher = DefaultHasher::new();
        SystemTime::now().hash(&mut hasher);
        let hash = hasher.finish();
        T::from(hash)
    }
}
