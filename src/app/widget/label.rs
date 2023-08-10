use ratatui::{
    style::{Modifier, Style, Color},
    widgets::Paragraph, prelude::{Layout, Direction, Constraint},
};

use super::Renderable;

pub struct Label {
    caption: Option<String>,
    text: Option<String>,
    fg_text: Color,
}

impl Label {
    pub fn new(caption: Option<String>, text: Option<String>) -> Self {
        Self { caption, text, fg_text: Color::White }
    }

    pub fn fg_text(mut self, color: Color) -> Self {
        self.fg_text = color;
        self
    }
}

impl Renderable for Label {
    fn render<B: ratatui::backend::Backend>(
        &mut self,
        f: &mut ratatui::Frame<B>,
        area: ratatui::layout::Rect,
    ) {
        // Layout
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(area);

        // Caption
        let caption = Paragraph::new(match &self.caption {
            Some(c) => &c,
            None => "",
        })
        .style(Style::default().remove_modifier(Modifier::BOLD));

        f.render_widget(caption, layout[0]);

        // Text
        let text = Paragraph::new(match &self.text {
            Some(t) => &t,
            None => "",
        })
        .style(Style::default().fg(self.fg_text));

        f.render_widget(text, layout[1]);
    }
}
