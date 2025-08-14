use rusty2048_core::{Game, GameConfig, Direction, GameState, GameStats};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction as LayoutDirection, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph, Clear},
    Terminal,
};

mod theme;
use theme::{ThemeManager, get_tile_color, get_tile_text_color, hex_to_color};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    cursor,
};
use std::{io, panic};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create a panic hook to restore terminal on panic
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        original_hook(panic_info);
    }));

    // Create game
    let config = GameConfig::default();
    let mut game = Game::new(config)?;

    // Run the game
    let res = run_game(&mut terminal, &mut game);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_game<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    game: &mut Game,
) -> io::Result<()> {
    let mut show_game_over = false;
    let mut show_win = false;
    let mut last_score = game.score().current();
    let mut score_animation = 0;
    let mut theme_manager = ThemeManager::new();
    let mut show_theme_help = false;
    
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(0),
                        Constraint::Length(5),
                    ]
                    .as_ref(),
                )
                .split(size);

            // Title
            let title = Paragraph::new(vec![Line::from(vec![Span::styled(
                format!("Rusty2048 - {}", theme_manager.current_theme_name()),
                Style::default()
                    .fg(hex_to_color(&theme_manager.current_theme.title_color))
                    .add_modifier(Modifier::BOLD),
            )])])
            .block(Block::default().borders(Borders::NONE));
            f.render_widget(title, chunks[0]);

            // Game board
            let board_chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .constraints(
                    (0..game.board().size())
                        .map(|_| Constraint::Length(3))
                        .collect::<Vec<_>>(),
                )
                .split(chunks[1]);

            for (row, &chunk) in board_chunks.iter().enumerate() {
                let row_chunks = Layout::default()
                    .direction(LayoutDirection::Horizontal)
                    .constraints(
                        (0..game.board().size())
                            .map(|_| Constraint::Length(8))
                            .collect::<Vec<_>>(),
                    )
                    .split(chunk);

                for (col, &cell) in row_chunks.iter().enumerate() {
                    let tile = game.board().get_tile(row, col).unwrap();
                    let text = if tile.is_empty() {
                        " ".to_string()
                    } else {
                        tile.value.to_string()
                    };

                    let tile_color = get_tile_color(tile.value, &theme_manager.current_theme);
                    let text_color = get_tile_text_color(tile.value, &theme_manager.current_theme);
                    
                    let style = Style::default()
                        .fg(text_color)
                        .bg(tile_color);

                    let cell_widget = Paragraph::new(text)
                        .block(Block::default().borders(Borders::ALL))
                        .style(style);
                    f.render_widget(cell_widget, cell);
                }
            }

            // Get game stats and check for score changes
            let stats = game.stats();
            let duration = format_duration(stats.duration);
            let current_score = game.score().current();
            
            // Check if score increased (for animation)
            if current_score > last_score {
                score_animation = 10; // Show animation for 10 frames
                last_score = current_score;
                // Play a bell sound for score increase
                print!("\x07");
            }
            
            if score_animation > 0 {
                score_animation -= 1;
            }
            
            // Status and controls
            let mut status_text = vec![
                Line::from(vec![
                    Span::raw("Score: "),
                    Span::styled(
                        game.score().current().to_string(),
                        if score_animation > 0 {
                            Style::default().fg(hex_to_color(&theme_manager.current_theme.score_color)).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(hex_to_color(&theme_manager.current_theme.score_color))
                        },
                    ),
                    Span::raw(" | Best: "),
                    Span::styled(
                        game.score().best().to_string(),
                        Style::default().fg(hex_to_color(&theme_manager.current_theme.best_score_color)),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("Moves: "),
                    Span::styled(
                        game.moves().to_string(),
                        Style::default().fg(hex_to_color(&theme_manager.current_theme.moves_color)),
                    ),
                    Span::raw(" | Time: "),
                    Span::styled(
                        duration,
                        Style::default().fg(hex_to_color(&theme_manager.current_theme.time_color)),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("Controls: "),
                    Span::styled("WASD/Arrow Keys", Style::default().fg(Color::White)),
                    Span::raw(" Move | "),
                    Span::styled("R", Style::default().fg(Color::White)),
                    Span::raw(" Restart | "),
                    Span::styled("U", Style::default().fg(Color::White)),
                    Span::raw(" Undo | "),
                    Span::styled("T", Style::default().fg(Color::White)),
                    Span::raw(" Theme | "),
                    Span::styled("H", Style::default().fg(Color::White)),
                    Span::raw(" Help | "),
                    Span::styled("Q", Style::default().fg(Color::White)),
                    Span::raw(" Quit"),
                ]),
            ];

            // Add game state messages
            match game.state() {
                GameState::Won => {
                    if !show_win {
                        show_win = true;
                    }
                    status_text.push(Line::from(vec![
                        Span::styled(
                            "🎉 Congratulations! You won! Press R to restart or continue playing",
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                        ),
                    ]));
                }
                GameState::GameOver => {
                    if !show_game_over {
                        show_game_over = true;
                    }
                    status_text.push(Line::from(vec![
                        Span::styled(
                            "💀 Game Over! Press R to restart",
                            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                        ),
                    ]));
                    
                    // Add final statistics
                    let final_score = game.score().current();
                    let best_score = game.score().best();
                    let total_moves = game.moves();
                    let max_tile = game.board().max_tile();
                    
                    status_text.push(Line::from(vec![
                        Span::raw("Final Score: "),
                        Span::styled(
                            final_score.to_string(),
                            Style::default().fg(hex_to_color(&theme_manager.current_theme.score_color)).add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(" | Max Tile: "),
                        Span::styled(
                            max_tile.to_string(),
                            Style::default().fg(hex_to_color(&theme_manager.current_theme.best_score_color)).add_modifier(Modifier::BOLD),
                        ),
                    ]));
                    
                    if final_score > 0 {
                        let avg_score_per_move = final_score as f64 / total_moves as f64;
                        status_text.push(Line::from(vec![
                            Span::raw("Avg Score per Move: "),
                            Span::styled(
                                format!("{:.1}", avg_score_per_move),
                                Style::default().fg(hex_to_color(&theme_manager.current_theme.moves_color)),
                            ),
                        ]));
                    }
                }
                GameState::Playing => {
                    show_game_over = false;
                    show_win = false;
                }
            }

            // Add theme help if requested
            if show_theme_help {
                status_text.push(Line::from(vec![
                    Span::styled(
                        "Available Themes: Classic, Dark, Neon, Retro, Pastel",
                        Style::default().fg(Color::Cyan),
                    ),
                ]));
                status_text.push(Line::from(vec![
                    Span::styled(
                        "Press T to cycle themes, or number keys 1-5 to select directly",
                        Style::default().fg(Color::Cyan),
                    ),
                ]));
            }

            let status = Paragraph::new(status_text)
                .block(Block::default().borders(Borders::NONE));
            f.render_widget(status, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    return Ok(());
                }
                KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('k') => {
                    if game.state() == GameState::Playing {
                        let _ = game.make_move(Direction::Up);
                    }
                }
                KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('j') => {
                    if game.state() == GameState::Playing {
                        let _ = game.make_move(Direction::Down);
                    }
                }
                KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('h') => {
                    if game.state() == GameState::Playing {
                        let _ = game.make_move(Direction::Left);
                    }
                }
                KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('l') => {
                    if game.state() == GameState::Playing {
                        let _ = game.make_move(Direction::Right);
                    }
                }
                KeyCode::Char('r') => {
                    let _ = game.new_game();
                    show_game_over = false;
                    show_win = false;
                }
                KeyCode::Char('u') => {
                    if game.state() == GameState::Playing {
                        let _ = game.undo();
                    }
                }
                KeyCode::Char('t') => {
                    theme_manager.next_theme();
                }
                KeyCode::Char('1') => {
                    theme_manager.set_theme("Classic");
                }
                KeyCode::Char('2') => {
                    theme_manager.set_theme("Dark");
                }
                KeyCode::Char('3') => {
                    theme_manager.set_theme("Neon");
                }
                KeyCode::Char('4') => {
                    theme_manager.set_theme("Retro");
                }
                KeyCode::Char('5') => {
                    theme_manager.set_theme("Pastel");
                }
                KeyCode::Char('h') => {
                    show_theme_help = !show_theme_help;
                }
                _ => {}
            }
        }
    }
}

fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    
    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{}:{:02}", minutes, secs)
    }
}
