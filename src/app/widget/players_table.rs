use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

use crate::core::model::{currency::Currency, player::Player};

use super::Renderable;

pub struct PlayersTable {
    state: TableState,
    players: Vec<Player>,
}

impl PlayersTable {
    pub fn new(players: Vec<Player>, selection: Option<usize>) -> Self {
        let mut state = TableState::default();
        state.select(selection);
        Self { state, players }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.players.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.players.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

impl Renderable for PlayersTable {
    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let rows: Vec<Row> = self
            .players
            .iter()
            .map(|p| {
                let price_delta_ratio = p.price_delta_ratio(Currency::Euro);
                let (last_game_scores, games_count, played_games_count, played_games_ratio) =
                    match &p.stats {
                        Some(s) => (
                            s.last_game_scores(),
                            Some(s.games.len() as u64),
                            s.played_games_count(),
                            s.played_games_ratio(),
                        ),
                        None => (None, None, None, None),
                    };

                Row::new(vec![
                    // Name
                    Cell::from(p.display_name.clone()),
                    // Team
                    Cell::from(match &p.team {
                        Some(t) => t.clone(),
                        None => "-".to_string(),
                    }),
                    // Score
                    Cell::from(match &p.stats {
                        Some(s) => format!("{}", s.score),
                        None => "-".to_string(),
                    }),
                    // Last Games Scores
                    Cell::from(match last_game_scores {
                        Some(s) => format!("{}", s.iter().map(|v| format!("{}", v)).collect::<Vec<String>>().join("-")),
                        None => "-".to_string(),
                    }),
                    // Last Games Played
                    Cell::from(format!(
                        "{}/{} ({})",
                        match played_games_count {
                            Some(c) => format!("{}", c),
                            None => "-".to_string(),
                        },
                        match games_count {
                            Some(c) => format!("{}", c),
                            None => "-".to_string(),
                        },
                        match played_games_ratio {
                            Some(c) => format!("{}%", (c * 100.0).round()),
                            None => "-".to_string(),
                        },
                    )),
                    // Price
                    Cell::from(format!(
                        "{} ({})",
                        match p.prices.get(0) {
                            Some(price) => format!("{} €", &price.eur),
                            None => "-".to_string(),
                        },
                        match price_delta_ratio {
                            Some(pd) => format!("{:.2}%", pd * 100.0),
                            None => "-".to_string(),
                        }
                    ))
                    .style(Style::default().fg(match price_delta_ratio {
                        Some(pd) => {
                            if pd >= 0.0 {
                                Color::Green
                            } else {
                                Color::Red
                            }
                        }
                        None => Color::Reset,
                    })),
                ])
            })
            .collect();

        let table = Table::new(rows)
            .header(
                Row::new(vec![
                    "Name",
                    "Team",
                    "Score",
                    "Last Games Scores",
                    "Last Games Played",
                    "Price",
                ])
                .style(Style::default().fg(Color::Yellow)), // If you want some space between the header and the rest of the rows, you can always
                                                            // specify some margin at the bottom.
                                                            //.bottom_margin(1),
            )
            // As any other widget, a Table can be wrapped in a Block.
            .block(
                Block::default()
                    .title("Players (⬆ & ⬇)")
                    .borders(Borders::ALL),
            )
            // Columns widths are constrained in the same way as Layout...
            .widths(&[
                Constraint::Percentage(20),
                Constraint::Percentage(16),
                Constraint::Percentage(10),
                Constraint::Percentage(22),
                Constraint::Percentage(16),
                Constraint::Percentage(16),
            ])
            // ...and they can be separated by a fixed spacing.
            .column_spacing(1)
            // If you wish to highlight a row in any specific way when it is selected...
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::White),
            )
            // ...and potentially show a symbol in front of the selection.
            .highlight_symbol(" ⛹️  ");

        f.render_stateful_widget(table, area, &mut self.state)
    }
}
