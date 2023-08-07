use ratatui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Row, Table, TableState},
    Frame,
};

use crate::core::model::decision::Decision;

use super::Renderable;

pub struct DecisionsTable {
    state: TableState,
    decisions: Vec<Decision>,
    focused: bool,
}

impl DecisionsTable {
    pub fn new(decisions: Vec<Decision>, selection: Option<usize>, focused: bool) -> Self {
        let mut state = TableState::default();
        state.select(selection);
        Self {
            state,
            decisions,
            focused,
        }
    }
}

impl Renderable for DecisionsTable {
    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let rows: Vec<Row> = self
            .decisions
            .iter()
            .map(|d| {
                Row::new(vec![
                    // Action
                    Cell::from(d.action.clone()),
                    // Player
                    Cell::from(d.player_name.clone()),
                    // Strategy
                    Cell::from(d.strategy.clone()),
                    //Comment
                    Cell::from(d.comment.clone()),
                ])
            })
            .collect();

        let table = Table::new(rows)
            .header(
                Row::new(vec!["Action", "Player", "Strategy", "Comment"])
                    .style(Style::default().fg(Color::Yellow)), // If you want some space between the header and the rest of the rows, you can always
                                                                // specify some margin at the bottom.
                                                                //.bottom_margin(1),
            )
            // As any other widget, a Table can be wrapped in a Block.
            .block(
                Block::default()
                    .title(if self.focused {
                        "Decisions (⬆⬇ to browse) (TAB to switch panel) (⌫  to clear)"
                    } else {
                        "Decisions"
                    })
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(if self.focused {
                        Color::Yellow
                    } else {
                        Color::Reset
                    })),
            )
            // Columns widths are constrained in the same way as Layout...
            .widths(&[
                Constraint::Percentage(10),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(50),
            ])
            // ...and they can be separated by a fixed spacing.
            .column_spacing(1)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::White),
            )
            // ...and potentially show a symbol in front of the selection.
            .highlight_symbol(" 🧠  ");

        f.render_stateful_widget(table, area, &mut self.state)
    }
}
