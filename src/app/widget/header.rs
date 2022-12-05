use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    widgets::Paragraph,
};

use super::Renderable;

pub struct Header {}

impl Renderable for Header {
    fn render<B: tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>, area: tui::layout::Rect) {
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
