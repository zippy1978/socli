use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use super::{
    state::{AppState, Panel},
    widget::{
        decisions_table::DecisionsTable, header::Header, logs_panel::LogsPanel,
        player_details::PlayerDetails, players_table::PlayersTable, Renderable,
    },
    App,
};

pub fn check_window_size(rect: &Rect) -> Result<(), String> {
    if rect.width < 52 {
        return Err(format!(
            "Terminal window too small, a width >= 52 is required, (got {})",
            rect.width
        ));
    }
    if rect.height < 28 {
        return Err(format!(
            "Terminal window too small, a height >= 28 is required, (got {})",
            rect.height
        ));
    }

    Ok(())
}

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();

    // Vertical layout
    let master_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Percentage(50),
                Constraint::Percentage(30),
                Constraint::Length(12),
            ]
            .as_ref(),
        )
        .split(size);

    // Header
    let mut header = Header {};
    header.render(rect, master_layout[0]);

    // Players horizontal layout (list + details)
    let player_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(master_layout[2]);

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
    player_table.render(rect, player_layout[0]);

    // Players details
    let mut player_details = if let AppState::Initialized {
        players,
        selected_player,
        selected_panel,
        ..
    } = &app.state
    {
        PlayerDetails::new(
            match players.get(*selected_player) {
                Some(p) => Some(p.clone()),
                None => None,
            },
            matches!(selected_panel, Panel::Player),
        )
    } else {
        PlayerDetails::new(None, false)
    };
    player_details.render(rect, player_layout[1]);

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
    decisons_table.render(rect, master_layout[3]);

    // Logs
    let mut logs_panel = if let AppState::Initialized { selected_panel, .. } = &app.state {
        LogsPanel::new(matches!(selected_panel, Panel::Logs))
    } else {
        LogsPanel::new(false)
    };
    logs_panel.render(rect, master_layout[4]);
}
