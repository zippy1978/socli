use tui::{
    style::{Color, Style},
    widgets::{Block, Borders, BorderType},
};
use tui_logger::TuiLoggerWidget;

use super::Renderable;

pub struct LogsPanel {}

impl Renderable for LogsPanel {
    fn render<B: tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>, area: tui::layout::Rect) {
        let widget = TuiLoggerWidget::default()
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Gray))
            .style_info(Style::default().fg(Color::Blue))
            .block(
                Block::default()
                    .title("Logs")
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL),
            );

            f.render_widget(widget, area)
    }
}
