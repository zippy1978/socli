use chrono::DateTime;
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{block::Title, Bar, BarChart, BarGroup, Block, BorderType, Borders, Padding},
};

use crate::core::model::{currency::Currency, player::Player};

use super::{label::Label, Renderable};

pub struct PlayerDetails {
    player: Option<Player>,
    focused: bool,
}

impl PlayerDetails {
    pub fn new(player: Option<Player>, focused: bool) -> Self {
        Self { player, focused }
    }

    fn render_summary<B: ratatui::backend::Backend>(
        &mut self,
        f: &mut ratatui::Frame<B>,
        area: ratatui::layout::Rect,
    ) {
        // Layout
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .margin(1)
            .split(area);

        // Block
        let block = Block::default()
            .borders(Borders::BOTTOM)
            .title(" 👤")
            .title(Title::from("Summary".fg(Color::Yellow)).alignment(Alignment::Left));
        f.render_widget(block, area);

        self.render_player_summary(f, layout[0]);
        self.render_price_summary(f, layout[1]);
    }

    fn render_player_summary<B: ratatui::backend::Backend>(
        &mut self,
        f: &mut ratatui::Frame<B>,
        area: ratatui::layout::Rect,
    ) {
        // Layout
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(area);

        // Name
        let mut name = Label::new(Some("Name".into()), None);
        if let Some(player) = &self.player {
            name = Label::new(Some("Name".into()), Some(player.display_name.clone()));
        }
        name.render(f, layout[0]);

        // Team
        let mut team = Label::new(Some("Team".into()), None);
        if let Some(player) = &self.player {
            if let Some(t) = &player.team {
                team = Label::new(Some("Team".into()), Some(t.clone()));
            }
        }
        team.render(f, layout[1]);

        // Age
        let mut age = Label::new(Some("Age".into()), None);
        if let Some(player) = &self.player {
            let birth_date = DateTime::parse_from_rfc3339(&player.birth_date).unwrap();
            let formatted_dob = birth_date.format("%m/%d/%Y");
            let text = format!("{} ({})", player.age(), formatted_dob,);
            age = Label::new(Some("Age".into()), Some(text));
        }
        age.render(f, layout[2]);

        // Score
        let mut score = Label::new(Some("Score".into()), None);
        if let Some(player) = &self.player {
            if let Some(stats) = &player.stats {
                score = Label::new(Some("Score".into()), Some(stats.score.to_string()));
            }
        }
        score.render(f, layout[3]);
    }

    fn render_price_summary<B: ratatui::backend::Backend>(
        &mut self,
        f: &mut ratatui::Frame<B>,
        area: ratatui::layout::Rect,
    ) {
        // Layout
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(area);

        // Floor price
        let mut floor_price = Label::new(Some("Floor Price".into()), Some("TODO".into()));
        floor_price.render(f, layout[0]);

        // Last sale
        let mut last_sale_price = Label::new(Some("Last Sale".into()), None);
        if let Some(player) = &self.player {
            if !player.prices.is_empty() {
                let price_delta_ratio = player.price_delta_ratio(Currency::Euro);
                let text = format!(
                    "{} ({})",
                    match player.prices.get(0) {
                        Some(price) => format!("{} €", &price.eur),
                        None => "-".to_string(),
                    },
                    match price_delta_ratio {
                        Some(pd) => format!("{:.2}%", pd * 100.0),
                        None => "-".to_string(),
                    }
                );
                last_sale_price = Label::new(Some("Last Sale".into()), Some(text)).fg_text(
                    match price_delta_ratio {
                        Some(d) => {
                            if d >= 0.0 {
                                Color::Green
                            } else {
                                Color::Red
                            }
                        }
                        None => Color::White,
                    },
                );
            }
        }
        last_sale_price.render(f, layout[1]);

        // Last 5 Sales average
        let mut avg_sale_price = Label::new(Some("Last 5 Sales Avg.".into()), None);
        if let Some(player) = &self.player {
            avg_sale_price = Label::new(
                Some("Last 5 Sales Avg.".into()),
                Some(match player.price_avg(Currency::Euro, 5) {
                    Some(avg) => format!("{:.2} €", avg),
                    None => "-".to_string(),
                }),
            );
        }
        avg_sale_price.render(f, layout[2]);

        // Liquidity
        let mut liquidity = Label::new(Some("Liquidity".into()), Some("TODO".into()));
        liquidity.render(f, layout[3]);
    }

    fn render_stats<B: ratatui::backend::Backend>(
        &mut self,
        f: &mut ratatui::Frame<B>,
        area: ratatui::layout::Rect,
    ) {
        let bars = match &self.player {
            Some(player) => match &player.stats {
                Some(stats) => stats
                    .games
                    .iter()
                    .rev()
                    .map(|g| {
                        let game_date = DateTime::parse_from_rfc3339(&g.date).unwrap();
                        let formatted = game_date.format("%m/%d").to_string();
                        Bar::default().value(g.score).label(formatted.into())
                    })
                    .collect::<Vec<Bar>>(),
                None => vec![],
            },
            None => vec![],
        };

        let bar_chart = BarChart::default()
            .data(BarGroup::default().bars(&bars))
            .style(Style::default())
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .padding(Padding::uniform(1))
                    .title(" 🏀")
                    .title(Title::from("Games".fg(Color::Yellow)).alignment(Alignment::Left)),
            )
            .bar_style(Style::default().fg(Color::Yellow))
            .value_style(Style::default().fg(Color::Black).bg(Color::Yellow))
            .bar_width((area.width) / 11);
        f.render_widget(bar_chart, area);
    }
}

impl Renderable for PlayerDetails {
    fn render<B: ratatui::backend::Backend>(
        &mut self,
        f: &mut ratatui::Frame<B>,
        area: ratatui::layout::Rect,
    ) {
        // Layout
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                ]
                .as_ref(),
            )
            .margin(1)
            .split(area);

        // Block
        let block = Block::default()
            .borders(Borders::ALL)
            .title(if self.focused {
                "Player (TAB to switch panel)"
            } else {
                "Player"
            })
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(if self.focused {
                Color::Yellow
            } else {
                Color::Reset
            }));
        f.render_widget(block, area);

        // Sub components
        self.render_summary(f, layout[0]);
        self.render_stats(f, layout[1]);
    }
}
