use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use super::{
    state::{AppState, Panel},
    widget::{
        decisions_table::DecisionsTable, header::Header, logs_panel::LogsPanel,
        players_table::PlayersTable, Renderable,
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
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Length(12),
            ]
            .as_ref(),
        )
        .split(size);

    // Header
    let mut header = Header {};
    header.render(rect, chunks[0]);

    // Players table
    let mut player_table = if let AppState::Initialized {
        players,
        selected_player,
        selected_panel,
        ..
    } = &app.state
    {
        PlayersTable::new(
            players.clone(),
            Some(*selected_player),
            matches!(selected_panel, Panel::Players),
        )
    } else {
        PlayersTable::new(vec![], None, false)
    };
    player_table.render(rect, chunks[2]);

    // Decisions
    let mut decisons_table = if let AppState::Initialized {
        decisions,
        selected_panel,
        selected_decision,
        ..
    } = &app.state
    {
        DecisionsTable::new(
            decisions.clone(),
            Some(*selected_decision),
            matches!(selected_panel, Panel::Decisions),
        )
    } else {
        DecisionsTable::new(vec![], None, false)
    };
    decisons_table.render(rect, chunks[3]);

    // Logs
    let mut logs_panel = if let AppState::Initialized { selected_panel, .. } = &app.state {
        LogsPanel::new(matches!(selected_panel, Panel::Logs))
    } else {
        LogsPanel::new(false)
    };
    logs_panel.render(rect, chunks[4]);
}
