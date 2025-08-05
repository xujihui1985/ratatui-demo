use ratatui::{
    prelude::{Alignment, Frame, Modifier, Style},
    style::Color,
    widgets::Paragraph,
};

use crate::model::AppState;

pub struct Status {
    pub state: AppState,
}

impl Status {
    pub fn new() -> Self {
        Self {
            state: AppState::Idle,
        }
    }

    pub fn view(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let status = Paragraph::new(match self.state {
            AppState::Idle => "Idle".to_string(),
            AppState::Busy => "Busy".to_string(),
            AppState::Quitting => "Quitting".to_string(),
        })
        .style(
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Right);
        f.render_widget(status, area);
    }
}
