use rusty2048_core::StatisticsManager;
use ratatui::{
    layout::{Constraint, Direction as LayoutDirection, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};


/// Chart display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartMode {
    /// Summary statistics
    Summary,
    /// Score trend chart
    ScoreTrend,
    /// Efficiency trend chart
    EfficiencyTrend,
    /// Tile achievements chart
    TileAchievements,
    /// Recent games table
    RecentGames,
}

/// Statistics charts display
pub struct ChartsDisplay {
    stats_manager: StatisticsManager,
    current_mode: ChartMode,
}

impl ChartsDisplay {
    /// Create a new charts display
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let stats_manager = StatisticsManager::new("cli/stats.json")?;
        Ok(Self {
            stats_manager,
            current_mode: ChartMode::Summary,
        })
    }
    
    /// Switch to next chart mode
    pub fn next_mode(&mut self) {
        self.current_mode = match self.current_mode {
            ChartMode::Summary => ChartMode::ScoreTrend,
            ChartMode::ScoreTrend => ChartMode::EfficiencyTrend,
            ChartMode::EfficiencyTrend => ChartMode::TileAchievements,
            ChartMode::TileAchievements => ChartMode::RecentGames,
            ChartMode::RecentGames => ChartMode::Summary,
        };
    }
    
    /// Switch to previous chart mode
    pub fn prev_mode(&mut self) {
        self.current_mode = match self.current_mode {
            ChartMode::Summary => ChartMode::RecentGames,
            ChartMode::ScoreTrend => ChartMode::Summary,
            ChartMode::EfficiencyTrend => ChartMode::ScoreTrend,
            ChartMode::TileAchievements => ChartMode::EfficiencyTrend,
            ChartMode::RecentGames => ChartMode::TileAchievements,
        };
    }
    
    /// Get current mode name
    pub fn mode_name(&self) -> &'static str {
        match self.current_mode {
            ChartMode::Summary => "Summary",
            ChartMode::ScoreTrend => "Score Trend",
            ChartMode::EfficiencyTrend => "Efficiency Trend",
            ChartMode::TileAchievements => "Tile Achievements",
            ChartMode::RecentGames => "Recent Games",
        }
    }
    
    /// Render the current chart
    pub fn render(&self, f: &mut Frame, area: Rect) {
        match self.current_mode {
            ChartMode::Summary => self.render_summary(f, area),
            ChartMode::ScoreTrend => self.render_score_trend(f, area),
            ChartMode::EfficiencyTrend => self.render_efficiency_trend(f, area),
            ChartMode::TileAchievements => self.render_tile_achievements(f, area),
            ChartMode::RecentGames => self.render_recent_games(f, area),
        }
    }
    
    /// Render summary statistics
    fn render_summary(&self, f: &mut Frame, area: Rect) {
        let summary = self.stats_manager.get_summary();
        
        let chunks = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ].as_ref())
            .split(area);
        
        // Title
        let title = Paragraph::new("📊 Statistics Summary")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(title, chunks[0]);
        
        // Summary content
        let mut summary_text = Vec::new();
        
        if summary.total_games == 0 {
            summary_text.push(Line::from(vec![
                Span::styled("No games played yet!", Style::default().fg(Color::Yellow))
            ]));
        } else {
            summary_text.push(Line::from(vec![
                Span::styled("Games Played: ", Style::default().fg(Color::White)),
                Span::styled(summary.total_games.to_string(), Style::default().fg(Color::Green)),
                Span::styled(" | Won: ", Style::default().fg(Color::White)),
                Span::styled(summary.games_won.to_string(), Style::default().fg(Color::Green)),
                Span::styled(" | Win Rate: ", Style::default().fg(Color::White)),
                Span::styled(format!("{:.1}%", summary.win_rate), Style::default().fg(Color::Green)),
            ]));
            
            summary_text.push(Line::from(vec![
                Span::styled("Highest Score: ", Style::default().fg(Color::White)),
                Span::styled(summary.highest_score.to_string(), Style::default().fg(Color::Yellow)),
                Span::styled(" | Average Score: ", Style::default().fg(Color::White)),
                Span::styled(format!("{:.0}", summary.average_score), Style::default().fg(Color::Yellow)),
            ]));
            
            summary_text.push(Line::from(vec![
                Span::styled("Total Moves: ", Style::default().fg(Color::White)),
                Span::styled(summary.total_moves.to_string(), Style::default().fg(Color::Blue)),
                Span::styled(" | Avg Moves: ", Style::default().fg(Color::White)),
                Span::styled(format!("{:.1}", summary.average_moves), Style::default().fg(Color::Blue)),
            ]));
            
            summary_text.push(Line::from(vec![
                Span::styled("Total Play Time: ", Style::default().fg(Color::White)),
                Span::styled(format_duration(summary.total_play_time), Style::default().fg(Color::Magenta)),
                Span::styled(" | Avg Duration: ", Style::default().fg(Color::White)),
                Span::styled(format_duration(summary.average_duration as u64), Style::default().fg(Color::Magenta)),
            ]));
            
            summary_text.push(Line::from(vec![
                Span::styled("Highest Tile: ", Style::default().fg(Color::White)),
                Span::styled(summary.highest_tile.to_string(), Style::default().fg(Color::Red)),
            ]));
            
            // Score distribution
            summary_text.push(Line::from(vec![
                Span::styled("Score Distribution:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            ]));
            
            summary_text.push(Line::from(vec![
                Span::styled("0-1000: ", Style::default().fg(Color::White)),
                Span::styled(summary.score_distribution.low_score.to_string(), Style::default().fg(Color::Red)),
                Span::styled(" | 1001-5000: ", Style::default().fg(Color::White)),
                Span::styled(summary.score_distribution.medium_score.to_string(), Style::default().fg(Color::Yellow)),
            ]));
            
            summary_text.push(Line::from(vec![
                Span::styled("5001-10000: ", Style::default().fg(Color::White)),
                Span::styled(summary.score_distribution.high_score.to_string(), Style::default().fg(Color::Green)),
                Span::styled(" | 10001+: ", Style::default().fg(Color::White)),
                Span::styled(summary.score_distribution.very_high_score.to_string(), Style::default().fg(Color::Cyan)),
            ]));
        }
        
        let summary_widget = Paragraph::new(summary_text)
            .block(Block::default().title("Statistics").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));
        f.render_widget(summary_widget, chunks[1]);
    }
    
    /// Render score trend chart
    fn render_score_trend(&self, f: &mut Frame, area: Rect) {
        let trend_data = self.stats_manager.get_score_trend(20);
        
        let chunks = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ].as_ref())
            .split(area);
        
        // Title
        let title = Paragraph::new("📈 Score Trend (Last 20 Games)")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(title, chunks[0]);
        
        if trend_data.is_empty() {
            let message = Paragraph::new("No data available")
                .style(Style::default().fg(Color::Yellow))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(message, chunks[1]);
            return;
        }
        
        // Create ASCII chart
        let max_score = trend_data.iter().map(|(_, score)| *score).max().unwrap_or(1);
        let chart_height = 15;
        let mut chart_lines = Vec::new();
        
        for row in 0..chart_height {
            let threshold = max_score * (chart_height - row) / chart_height;
            let mut line = format!("{:>8} |", threshold);
            
            for (_, score) in &trend_data {
                if *score >= threshold {
                    line.push_str(" █");
                } else {
                    line.push_str("  ");
                }
            }
            
            chart_lines.push(Line::from(vec![
                Span::styled(line, Style::default().fg(Color::Green))
            ]));
        }
        
        // Add x-axis
        let mut x_axis = "         |".to_string();
        for i in 0..trend_data.len() {
            if i % 5 == 0 {
                x_axis.push_str(&format!("{:2}", i));
            } else {
                x_axis.push_str("  ");
            }
        }
        chart_lines.push(Line::from(vec![
            Span::styled(x_axis, Style::default().fg(Color::White))
        ]));
        
        let chart_widget = Paragraph::new(chart_lines)
            .block(Block::default().title("Score Trend Chart").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));
        f.render_widget(chart_widget, chunks[1]);
    }
    
    /// Render efficiency trend chart
    fn render_efficiency_trend(&self, f: &mut Frame, area: Rect) {
        let trend_data = self.stats_manager.get_efficiency_trend(20);
        
        let chunks = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ].as_ref())
            .split(area);
        
        // Title
        let title = Paragraph::new("📊 Efficiency Trend (Last 20 Games)")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(title, chunks[0]);
        
        if trend_data.is_empty() {
            let message = Paragraph::new("No data available")
                .style(Style::default().fg(Color::Yellow))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(message, chunks[1]);
            return;
        }
        
        // Create ASCII chart
        let max_efficiency = trend_data.iter().map(|(_, eff)| *eff).fold(0.0, f64::max);
        let chart_height = 15;
        let mut chart_lines = Vec::new();
        
        for row in 0..chart_height {
            let threshold = max_efficiency * (chart_height - row) as f64 / chart_height as f64;
            let mut line = format!("{:>8.0} |", threshold);
            
            for (_, efficiency) in &trend_data {
                if *efficiency >= threshold {
                    line.push_str(" █");
                } else {
                    line.push_str("  ");
                }
            }
            
            chart_lines.push(Line::from(vec![
                Span::styled(line, Style::default().fg(Color::Blue))
            ]));
        }
        
        // Add x-axis
        let mut x_axis = "         |".to_string();
        for i in 0..trend_data.len() {
            if i % 5 == 0 {
                x_axis.push_str(&format!("{:2}", i));
            } else {
                x_axis.push_str("  ");
            }
        }
        chart_lines.push(Line::from(vec![
            Span::styled(x_axis, Style::default().fg(Color::White))
        ]));
        
        let chart_widget = Paragraph::new(chart_lines)
            .block(Block::default().title("Efficiency Trend Chart").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));
        f.render_widget(chart_widget, chunks[1]);
    }
    
    /// Render tile achievements chart
    fn render_tile_achievements(&self, f: &mut Frame, area: Rect) {
        let tile_data = self.stats_manager.get_tile_achievements();
        
        let chunks = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ].as_ref())
            .split(area);
        
        // Title
        let title = Paragraph::new("🏆 Tile Achievements")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(title, chunks[0]);
        
        if tile_data.is_empty() {
            let message = Paragraph::new("No data available")
                .style(Style::default().fg(Color::Yellow))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(message, chunks[1]);
            return;
        }
        
        let max_count = tile_data.iter().map(|(_, count)| *count).max().unwrap_or(1);
        let mut chart_lines = Vec::new();
        
        for (tile, count) in &tile_data {
            let bar_length = if max_count > 0 {
                (count * 20) / max_count
            } else {
                0
            };
            
            let bar = "█".repeat(bar_length as usize);
            let line = format!("{:>6} | {:>3} | {}", tile, count, bar);
            
            chart_lines.push(Line::from(vec![
                Span::styled(line, Style::default().fg(Color::Yellow))
            ]));
        }
        
        let chart_widget = Paragraph::new(chart_lines)
            .block(Block::default().title("Tile Achievement Chart").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));
        f.render_widget(chart_widget, chunks[1]);
    }
    
    /// Render recent games table
    fn render_recent_games(&self, f: &mut Frame, area: Rect) {
        let summary = self.stats_manager.get_summary();
        
        let chunks = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ].as_ref())
            .split(area);
        
        // Title
        let title = Paragraph::new("📋 Recent Games")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(title, chunks[0]);
        
        if summary.recent_games.is_empty() {
            let message = Paragraph::new("No recent games")
                .style(Style::default().fg(Color::Yellow))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(message, chunks[1]);
            return;
        }
        
        let rows: Vec<Row> = summary.recent_games
            .iter()
            .map(|game| {
                let status = if game.won { "Won" } else { "Lost" };
                let duration = format_duration(game.duration);
                
                Row::new(vec![
                    game.final_score.to_string(),
                    game.moves.to_string(),
                    game.max_tile.to_string(),
                    status.to_string(),
                    duration,
                    format!("{:.1}", game.efficiency),
                ])
            })
            .collect();
        
        let table = Table::new(rows, &[
            Constraint::Length(10), // Score
            Constraint::Length(8),  // Moves
            Constraint::Length(8),  // Max Tile
            Constraint::Length(6),  // Status
            Constraint::Length(10), // Duration
            Constraint::Length(10), // Efficiency
        ])
        .header(Row::new(vec!["Score", "Moves", "Max Tile", "Status", "Duration", "Efficiency"]))
        .block(Block::default().title("Recent Games").borders(Borders::ALL));
        
        f.render_widget(table, chunks[1]);
    }
    
    /// Get statistics manager reference
    pub fn stats_manager(&mut self) -> &mut StatisticsManager {
        &mut self.stats_manager
    }
}

/// Format duration in seconds to human readable format
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
