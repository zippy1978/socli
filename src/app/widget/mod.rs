use tui::{backend::Backend, Frame, layout::Rect};

pub mod players_table;
pub mod logs_panel;
pub mod decisions_table;
pub mod header;

pub trait Renderable {
    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect);
}