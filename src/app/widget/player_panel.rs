use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{BarChart, Block, Borders, Paragraph},
};

use crate::core::model::player::Player;

use super::Renderable;

pub struct PlayerPanel {
    player: Option<Player>,
}

impl PlayerPanel {
    pub fn new(player: Option<Player>) -> Self {
        Self { player }
    }
}

impl Renderable for PlayerPanel {
    fn render<B: tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>, area: tui::layout::Rect) {
        // Vertical layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(40),
                    Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(area);

        if let Some(player) = &self.player {
            // Title
            let widget = Paragraph::new(player.display_name.clone())
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center);

            f.render_widget(widget, chunks[0]);

            // Prices
            let formatted_data: Vec<(String, u64)> = player
                .prices
                .iter()
                .map(|p| {
                    let time = iso8601::datetime(&p.date).unwrap().time;
                    let label = format!("{}@{:<02}:{:<02}", p.eur, time.hour, time.minute);
                    (label, (p.eur.parse::<f64>().unwrap() * 100.0) as u64)
                })
                .rev()
                .collect();
            let data: Vec<(&str, u64)> =
                formatted_data.iter().map(|p| (p.0.as_str(), p.1)).collect();

            let max = data.iter().map(|d| d.1).max().unwrap_or(0);
            let chart = BarChart::default()
                .block(Block::default().title("Prices (€)").borders(Borders::NONE))
                .style(Style::default().fg(Color::Yellow))
                .value_style(Style::default().fg(Color::Yellow).bg(Color::Yellow))
                .label_style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .bar_style(Style::default().fg(Color::Yellow))
                .data(&data)
                .bar_width(11)
                .bar_gap(4)
                .max(max);
            f.render_widget(chart.clone(), chunks[1]);

            // Test
            f.render_widget(chart, chunks[2]);
        } else {
            f.render_widget(Paragraph::new(""), chunks[0])
        }
    }
}
