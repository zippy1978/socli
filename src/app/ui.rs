use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::{
    state::AppState,
    widget::{
        decisions_table::DecisionsTable, logs_panel::LogsPanel, players_table::PlayersTable,
        Renderable,
    },
    App,
};

fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();
    check_size(&size);

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Length(12),
            ]
            .as_ref(),
        )
        .split(size);

    // Title
    let title = draw_title();
    rect.render_widget(title, chunks[0]);

    // Players table
    let mut player_table = if let AppState::Initialized {
        players,
        selected_player,
        ..
    } = &app.state
    {
        PlayersTable::new(players.clone(), Some(*selected_player))
    } else {
        PlayersTable::new(vec![], None)
    };
    player_table.render(rect, chunks[1]);

    // Decisions
    let mut decisons_table = if let AppState::Initialized {
        decisions,
        ..
    } = &app.state
    {
        DecisionsTable::new(decisions.clone())
    } else {
        DecisionsTable::new(vec![])
    };
    decisons_table.render(rect, chunks[2]);

    // Logs
    let mut logs_panel = LogsPanel {};
    logs_panel.render(rect, chunks[3]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("SoCli - Sorare CLI")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}
