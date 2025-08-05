
use ratatui::{
    prelude::Frame,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::model::Message;

pub struct Conversation {
    pub conversation: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Self {
        Self {
            conversation: Vec::new(),
        }
    }

    pub fn view(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let mut conversation_text = Vec::new();
        for message in &self.conversation {
            match message {
                Message::User(text) => {
                    conversation_text.push(Line::from(vec![
                        Span::styled("You: ", Style::default().fg(Color::Yellow)),
                        Span::raw(text),
                    ]));
                }
                Message::Model(text) => {
                    conversation_text.push(Line::from(vec![
                        Span::styled("Gemini: ", Style::default().fg(Color::Green)),
                        Span::raw(text),
                    ]));
                }
                Message::ToolCode(text) => {
                    let block = Block::new()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(Color::Blue));
                    let _paragraph = Paragraph::new(text.clone()).block(block);
                    conversation_text.push(Line::from(vec![Span::styled(
                        "Tool Code:",
                        Style::default().fg(Color::Blue),
                    )]));
                    conversation_text.push(Line::from(vec![Span::raw(text.clone())]));
                }
            }
        }
        let conversation = Paragraph::new(conversation_text)
            .block(
                Block::default()
                    .title("Conversation")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .wrap(Wrap { trim: true });
        f.render_widget(conversation, area);
    }
}
