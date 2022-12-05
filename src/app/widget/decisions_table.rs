use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::core::model::decision::Decision;

use super::Renderable;

pub struct DecisionsTable {
    decisions: Vec<Decision>,
}

impl DecisionsTable {
    pub fn new(decisions: Vec<Decision>) -> Self {
        Self { decisions }
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
            .block(Block::default().title("Decisions").borders(Borders::ALL))
            // Columns widths are constrained in the same way as Layout...
            .widths(&[
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(70),
            ])
            // ...and they can be separated by a fixed spacing.
            .column_spacing(1);

        f.render_widget(table, area)
    }
}
