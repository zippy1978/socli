use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    widgets::Paragraph,
};

use super::Renderable;

pub struct Header {}

impl Renderable for Header {
    fn render<B: ratatui::backend::Backend>(&mut self, f: &mut ratatui::Frame<B>, area: ratatui::layout::Rect) {
        let widget = Paragraph::new(format!(
            "SoCli - A Sorare NBA 🏀 CLI - {}",
            env!("CARGO_PKG_VERSION")
        ))
        .style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);

        f.render_widget(widget, area)
    }
}
