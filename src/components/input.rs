use ratatui::{
    prelude::{Frame, Style},
    style::Color,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::model::AppState;

pub struct Input {
    pub input: String,
    pub state: AppState,
}

impl Input {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            state: AppState::Idle,
        }
    }

    pub fn view(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let input = Paragraph::new(self.input.as_str())
            .block(
                Block::default()
                    .title("Input")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(match self.state {
                AppState::Idle => Style::default(),
                AppState::Busy => Style::default().fg(Color::DarkGray),
                AppState::Quitting => Style::default(),
            });
        f.render_widget(input, area);
        f.set_cursor_position((area.x + self.input.len() as u16 + 1, area.y + 1));
    }
}
