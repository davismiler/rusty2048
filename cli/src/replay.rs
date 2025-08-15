use rusty2048_core::{ReplayRecorder, ReplayPlayer, ReplayData, ReplayMetadata, GameConfig, Direction};

use crate::theme::ThemeManager;

const REPLAY_DIR: &str = "replays";
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
};
use ratatui::{

    layout::{Constraint, Direction as LayoutDirection, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Terminal,
};
use serde_json;
use std::{
    fs,
    io,
    time::{Duration, Instant},
};

/// Replay mode for CLI
pub struct ReplayMode {
    /// Current replay recorder (if recording)
    recorder: Option<ReplayRecorder>,
    /// Current replay player (if playing)
    player: Option<ReplayPlayer>,
    /// Theme manager
    theme_manager: ThemeManager,
    /// Current mode
    mode: ReplayModeState,
    /// Auto-play interval
    auto_play_interval: Duration,
    /// Last auto-play time
    last_auto_play: Instant,
}

#[derive(Debug, Clone)]
enum ReplayModeState {
    Menu,
    Recording,
    Playing,
    LoadReplay,
}

impl ReplayMode {
    /// Create a new replay mode
    pub fn new() -> io::Result<Self> {
        // Ensure replay directory exists
        Self::ensure_replay_dir()?;
        
        Ok(Self {
            recorder: None,
            player: None,
            theme_manager: ThemeManager::new(),
            mode: ReplayModeState::Menu,
            auto_play_interval: Duration::from_millis(500),
            last_auto_play: Instant::now(),
        })
    }
    
    /// Ensure the replay directory exists
    fn ensure_replay_dir() -> io::Result<()> {
        if !fs::metadata(REPLAY_DIR).is_ok() {
            fs::create_dir(REPLAY_DIR)?;
        }
        Ok(())
    }
    
    /// Run the replay mode
    pub fn run<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        terminal.clear()?;
        
        loop {
            match self.mode {
                ReplayModeState::Menu => {
                    if !self.show_menu(terminal)? {
                        break;
                    }
                }
                ReplayModeState::Recording => {
                    if !self.handle_recording(terminal)? {
                        self.mode = ReplayModeState::Menu;
                    }
                }
                ReplayModeState::Playing => {
                    if !self.handle_playing(terminal)? {
                        self.mode = ReplayModeState::Menu;
                    }
                }
                ReplayModeState::LoadReplay => {
                    if !self.handle_load_replay(terminal)? {
                        self.mode = ReplayModeState::Menu;
                    }
                }
            }
        }
        
        terminal.clear()?;
        Ok(())
    }
    
    /// Show the main menu
    fn show_menu<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<bool> {
        let theme = &self.theme_manager.current_theme;
        
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(8),
                    Constraint::Min(0),
                ].as_ref())
                .split(size);
            
            // Title
            let title = Paragraph::new("🎬 Rusty2048 Replay System")
                .style(Style::default()
                    .fg(crate::theme::hex_to_color(&theme.title_color))
                    .add_modifier(Modifier::BOLD))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(title, chunks[0]);
            
            // Menu options
            let menu_items = vec![
                "1. Start Recording New Game",
                "2. Load and Play Replay",
                "3. List Saved Replays",
                "4. Back to Main Menu",
            ];
            
            let menu_text: Vec<Line> = menu_items
                .iter()
                .map(|item| Line::from(vec![Span::styled(
                    *item,
                    Style::default().fg(crate::theme::hex_to_color(&theme.text_color))
                )]))
                .collect();
            
            let menu = Paragraph::new(menu_text)
                .block(Block::default()
                    .title("Menu")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(crate::theme::hex_to_color(&theme.text_color))))
                .style(Style::default().fg(crate::theme::hex_to_color(&theme.text_color)));
            f.render_widget(menu, chunks[1]);
            
            // Instructions
            let instructions = Paragraph::new(vec![
                Line::from(vec![Span::styled(
                    "Use number keys (1-4) to select an option",
                    Style::default().fg(Color::Yellow)
                )]),
                Line::from(vec![Span::styled(
                    "Press 'q' to quit",
                    Style::default().fg(Color::Yellow)
                )]),
            ]);
            f.render_widget(instructions, chunks[2]);
        })?;
        
        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('1') => {
                        self.start_recording()?;
                        self.mode = ReplayModeState::Recording;
                    }
                    KeyCode::Char('2') => {
                        self.mode = ReplayModeState::LoadReplay;
                    }
                    KeyCode::Char('3') => {
                        self.list_replays(terminal)?;
                    }
                    KeyCode::Char('4') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                }
            }
        }
        
        Ok(true)
    }
    
    /// Start recording a new game
    fn start_recording(&mut self) -> io::Result<()> {
        let config = GameConfig::default();
        self.recorder = Some(ReplayRecorder::new(config).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to create recorder: {}", e))
        })?);
        
        Ok(())
    }
    
    /// Handle recording mode
    fn handle_recording<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<bool> {
        // Get game state before drawing
        let game_state = {
            let recorder = self.recorder.as_mut().unwrap();
            let game = recorder.game();
            (
                game.score().current(),
                game.score().best(),
                game.moves(),
                game.state(),
                game.board().to_vec(),
            )
        };
        
        // Get current theme
        let theme = &self.theme_manager.current_theme;
        
        // Display current game state
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(16), // Increased from 10 to 16 for better square display
                    Constraint::Length(3),
                    Constraint::Min(0),
                ].as_ref())
                .split(size);
            
            // Title
            let title = Paragraph::new("🎥 Recording Game")
                .style(Style::default()
                    .fg(crate::theme::hex_to_color(&theme.title_color))
                    .add_modifier(Modifier::BOLD))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(title, chunks[0]);
            
            // Game board
            self.render_game_board_from_data(f, &game_state.4, chunks[1]);
            
            // Stats
            let stats = vec![
                format!("Score: {}", game_state.0),
                format!("Best: {}", game_state.1),
                format!("Moves: {}", game_state.2),
                format!("State: {:?}", game_state.3),
            ];
            
            let stats_text: Vec<Line> = stats
                .iter()
                .map(|stat| Line::from(vec![Span::styled(
                    stat.clone(),
                    Style::default().fg(crate::theme::hex_to_color(&theme.text_color))
                )]))
                .collect();
            
            let stats_widget = Paragraph::new(stats_text)
                .block(Block::default()
                    .title("Stats")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(crate::theme::hex_to_color(&theme.text_color))))
                .style(Style::default().fg(crate::theme::hex_to_color(&theme.text_color)));
            f.render_widget(stats_widget, chunks[2]);
            
            // Instructions
            let instructions = Paragraph::new(vec![
                Line::from(vec![Span::styled(
                    "Use arrow keys or WASD to move",
                    Style::default().fg(Color::Yellow)
                )]),
                Line::from(vec![Span::styled(
                    "Press 's' to stop recording, 'q' to quit",
                    Style::default().fg(Color::Yellow)
                )]),
            ]);
            f.render_widget(instructions, chunks[3]);
        })?;
        
        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                let recorder = self.recorder.as_mut().unwrap();
                match code {
                    KeyCode::Char('s') => {
                        self.stop_recording()?;
                        return Ok(false);
                    }
                    KeyCode::Char('q') => {
                        self.recorder = None;
                        return Ok(false);
                    }
                    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                        let _ = recorder.make_move(Direction::Up);
                    }
                    KeyCode::Down | KeyCode::Char('S') => {
                        let _ = recorder.make_move(Direction::Down);
                    }
                    KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                        let _ = recorder.make_move(Direction::Left);
                    }
                    KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                        let _ = recorder.make_move(Direction::Right);
                    }
                    _ => {}
                }
            }
        }
        
        Ok(true)
    }
    
    /// Stop recording and save replay
    fn stop_recording(&mut self) -> io::Result<()> {
        if let Some(mut recorder) = self.recorder.take() {
            let replay_data = recorder.stop_recording();
            
            // Use default name for now (can be enhanced later with TUI input)
            let metadata = ReplayMetadata::default();
            let mut replay_data = replay_data;
            replay_data.metadata = metadata;
            
            // Save replay
            let filename = format!("replay_{}.json", replay_data.metadata.created_at);
            let filepath = format!("{}/{}", REPLAY_DIR, filename);
            let json = serde_json::to_string_pretty(&replay_data).map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Failed to serialize replay: {}", e))
            })?;
            
            fs::write(&filepath, json)?;
            
            // Show success message in TUI
            // Note: This will be called from within a terminal context
            // We'll handle this differently to avoid terminal conflicts
        }
        
        Ok(())
    }
    
    /// Show save success message
    #[allow(dead_code)]
    fn show_save_success<B: ratatui::backend::Backend>(&self, filename: &str, terminal: &mut Terminal<B>) -> io::Result<()> {
        
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(5),
                    Constraint::Min(0),
                ].as_ref())
                .split(size);
            
            // Title
            let title = Paragraph::new("✅ Replay Saved Successfully!")
                .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(title, chunks[0]);
            
            // Message
            let message = Paragraph::new(vec![
                Line::from(format!("Replay saved as: {}", filename)),
                Line::from("Press any key to continue..."),
            ])
            .block(Block::default().title("Success").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));
            f.render_widget(message, chunks[1]);
        })?;
        
        // Wait for key press
        event::read()?;
        
        Ok(())
    }
    
    /// Handle load replay mode
    fn handle_load_replay<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<bool> {
        let theme = &self.theme_manager.current_theme;
        
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(10),
                    Constraint::Length(3),
                ].as_ref())
                .split(size);
            
            // Title
            let title = Paragraph::new("📁 Load Replay")
                .style(Style::default()
                    .fg(crate::theme::hex_to_color(&theme.title_color))
                    .add_modifier(Modifier::BOLD))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(title, chunks[0]);
            
            // File list
            let files = self.get_replay_files();
            let rows: Vec<Row> = files
                .iter()
                .enumerate()
                .map(|(i, file)| {
                    Row::new(vec![
                        format!("{}", i + 1),
                        file.clone(),
                    ])
                })
                .collect();
            
            let table = Table::new(rows, &[Constraint::Length(3), Constraint::Min(0)])
                .header(Row::new(vec!["#", "Filename"]))
                .block(Block::default()
                    .title("Available Replays")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(crate::theme::hex_to_color(&theme.text_color))));
            f.render_widget(table, chunks[1]);
            
            // Instructions
            let instructions = Paragraph::new(vec![
                Line::from(vec![Span::styled(
                    "Enter replay number to load",
                    Style::default().fg(Color::Yellow)
                )]),
                Line::from(vec![Span::styled(
                    "Press 'q' to go back",
                    Style::default().fg(Color::Yellow)
                )]),
            ]);
            f.render_widget(instructions, chunks[2]);
        })?;
        
        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        let index = c.to_digit(10).unwrap() as usize - 1;
                        let files = self.get_replay_files();
                        if index < files.len() {
                            if let Err(e) = self.load_replay(&files[index]) {
                                println!("Error loading replay: {}", e);
                            } else {
                                self.mode = ReplayModeState::Playing;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        
        Ok(true)
    }
    
    /// Handle playing mode
    fn handle_playing<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<bool> {
        // Handle auto-play logic first
        {
            let player = self.player.as_mut().unwrap();
            if player.is_playing() && self.last_auto_play.elapsed() >= self.auto_play_interval {
                if let Ok(true) = player.next_move() {
                    self.last_auto_play = Instant::now();
                } else {
                    player.pause();
                }
            }
        }
        
        // Get player state for display
        let player_state = {
            let player = self.player.as_ref().unwrap();
            let game = player.current_game();
            (
                game.board().to_vec(),
                player.progress(),
                player.current_move_index(),
                player.total_moves(),
                player.speed(),
                player.is_playing(),
            )
        };
        
        // Get current theme
        let theme = &self.theme_manager.current_theme;
        
        // Display current game state
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(16), // Increased from 10 to 16 for better square display
                    Constraint::Length(5),
                    Constraint::Min(0),
                ].as_ref())
                .split(size);
            
            // Title
            let title = Paragraph::new("▶️ Playing Replay")
                .style(Style::default()
                    .fg(crate::theme::hex_to_color(&theme.title_color))
                    .add_modifier(Modifier::BOLD))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(title, chunks[0]);
            
            // Game board
            self.render_game_board_from_data(f, &player_state.0, chunks[1]);
            
            // Replay controls
            let controls = vec![
                format!("Progress: {:.1}%", player_state.1),
                format!("Move: {}/{}", player_state.2, player_state.3),
                format!("Speed: {}x", player_state.4),
                format!("Status: {}", if player_state.5 { "Playing" } else { "Paused" }),
            ];
            
            let controls_text: Vec<Line> = controls
                .iter()
                .map(|control| Line::from(vec![Span::styled(
                    control.clone(),
                    Style::default().fg(crate::theme::hex_to_color(&theme.text_color))
                )]))
                .collect();
            
            let controls_widget = Paragraph::new(controls_text)
                .block(Block::default()
                    .title("Replay Controls")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(crate::theme::hex_to_color(&theme.text_color))))
                .style(Style::default().fg(crate::theme::hex_to_color(&theme.text_color)));
            f.render_widget(controls_widget, chunks[2]);
            
            // Instructions
            let instructions = Paragraph::new(vec![
                Line::from(vec![Span::styled(
                    "Space: Play/Pause, Left/Right: Step, +/-: Speed, q: Quit",
                    Style::default().fg(Color::Yellow)
                )]),
            ]);
            f.render_widget(instructions, chunks[3]);
        })?;
        
        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                let player = self.player.as_mut().unwrap();
                match code {
                    KeyCode::Char('q') => {
                        self.player = None;
                        return Ok(false);
                    }
                    KeyCode::Char(' ') => {
                        if player.is_playing() {
                            player.pause();
                        } else {
                            player.play();
                        }
                    }
                    KeyCode::Left => {
                        let _ = player.previous_move();
                    }
                    KeyCode::Right => {
                        let _ = player.next_move();
                    }
                    KeyCode::Char('+') | KeyCode::Char('=') => {
                        player.set_speed(player.speed() + 0.5);
                    }
                    KeyCode::Char('-') => {
                        player.set_speed((player.speed() - 0.5).max(0.1));
                    }
                    _ => {}
                }
            }
        }
        
        Ok(true)
    }
    
    /// Render game board from board data
    fn render_game_board_from_data(
        &self,
        f: &mut ratatui::Frame,
        board_data: &Vec<Vec<u32>>,
        area: ratatui::layout::Rect,
    ) {
        let size = board_data.len();
        let theme = &self.theme_manager.current_theme;
        
        // Create layout for the game board
        let board_chunks = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints(
                (0..size)
                    .map(|_| Constraint::Length(3))
                    .collect::<Vec<_>>(),
            )
            .split(area);

        for (row, &chunk) in board_chunks.iter().enumerate() {
            let row_chunks = Layout::default()
                .direction(LayoutDirection::Horizontal)
                .constraints(
                    (0..size)
                        .map(|_| Constraint::Length(8))
                        .collect::<Vec<_>>(),
                )
                .split(chunk);

            for (col, &cell) in row_chunks.iter().enumerate() {
                let value = board_data[row][col];
                let text = if value == 0 {
                    " ".to_string()
                } else {
                    value.to_string()
                };

                let tile_color = crate::theme::get_tile_color(value, &theme);
                let text_color = crate::theme::get_tile_text_color(value, &theme);
                
                let style = Style::default()
                    .fg(text_color)
                    .bg(tile_color);

                let cell_widget = Paragraph::new(text)
                    .block(Block::default().borders(Borders::ALL))
                    .style(style);
                f.render_widget(cell_widget, cell);
            }
        }
    }
    
    /// Render game board
    #[allow(dead_code)]
    fn render_game_board(
        &self,
        f: &mut ratatui::Frame,
        game: &rusty2048_core::Game,
        area: ratatui::layout::Rect,
    ) {
        let board = game.board();
        self.render_game_board_from_data(f, &board.to_vec(), area);
    }
    
    /// Get list of replay files
    fn get_replay_files(&self) -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(REPLAY_DIR) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.ends_with(".json") && name.starts_with("replay_") {
                            files.push(name.to_string());
                        }
                    }
                }
            }
        }
        files.sort();
        files
    }
    
    /// Load a replay file
    fn load_replay(&mut self, filename: &str) -> io::Result<()> {
        let filepath = format!("{}/{}", REPLAY_DIR, filename);
        let content = fs::read_to_string(&filepath)?;
        let replay_data: ReplayData = serde_json::from_str(&content).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to parse replay: {}", e))
        })?;
        
        self.player = Some(ReplayPlayer::new(replay_data).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to create player: {}", e))
        })?);
        
        Ok(())
    }
    
    /// List saved replays
    fn list_replays<B: ratatui::backend::Backend>(&self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let theme = &self.theme_manager.current_theme;
        
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ].as_ref())
                .split(size);
            
            // Title
            let title = Paragraph::new("📁 Saved Replays")
                .style(Style::default()
                    .fg(crate::theme::hex_to_color(&theme.title_color))
                    .add_modifier(Modifier::BOLD))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(title, chunks[0]);
            
            // File list
            let files = self.get_replay_files();
            if files.is_empty() {
                let message = Paragraph::new("No replay files found.")
                    .style(Style::default().fg(crate::theme::hex_to_color(&theme.text_color)))
                    .alignment(ratatui::layout::Alignment::Center);
                f.render_widget(message, chunks[1]);
            } else {
                let rows: Vec<Row> = files
                    .iter()
                    .enumerate()
                    .map(|(i, file)| {
                        Row::new(vec![
                            format!("{}", i + 1),
                            file.clone(),
                        ])
                    })
                    .collect();
                
                let table = Table::new(rows, &[Constraint::Length(3), Constraint::Min(0)])
                    .header(Row::new(vec!["#", "Filename"]))
                    .block(Block::default()
                        .title("Available Replays")
                        .borders(Borders::ALL)
                        .style(Style::default().fg(crate::theme::hex_to_color(&theme.text_color))));
                f.render_widget(table, chunks[1]);
            }
            
            // Instructions
            let instructions = Paragraph::new(vec![
                Line::from(vec![Span::styled(
                    "Press any key to continue...",
                    Style::default().fg(Color::Yellow)
                )]),
            ])
            .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(instructions, chunks[2]);
        })?;
        
        // Wait for key press
        event::read()?;
        
        Ok(())
    }
}
