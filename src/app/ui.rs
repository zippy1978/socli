use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::{
    state::AppState,
    widget::{logs_panel::LogsPanel, players_table::PlayersTable, Renderable,  player_panel::PlayerPanel},
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
                Constraint::Min(10),
                Constraint::Length(12),
            ]
            .as_ref(),
        )
        .split(size);

    // Title
    let title = draw_title();
    rect.render_widget(title, chunks[0]);

    // Body & Player & Help
    /*let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);*/

    // Players table
    let mut player_table = match &app.state {
        AppState::Initialized {
            players,
            selected_player,
        } => PlayersTable::new(players.clone(), Some(*selected_player)),
        _ => PlayersTable::new(vec![], None),
    };
    player_table.render(rect, chunks[1]);

    // Player
    /*let selected_player = match app.state() {
        
        AppState::Initialized { players, selected_player } => {
            players.get(*selected_player)
        },
        _ => None,
    };
    let mut player_panel = PlayerPanel::new(selected_player.cloned());
    player_panel.render(rect, body_chunks[1]);*/

    // Logs
    let mut logs_panel = LogsPanel {};
    logs_panel.render(rect, chunks[2]);
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
