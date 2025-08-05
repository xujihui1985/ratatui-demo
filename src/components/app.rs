use crate::components::{conversation::Conversation, input::Input, status::Status};
use crate::model::{AppState, Message};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::{Constraint, Direction, Frame, Layout};

pub struct App {
    conversation: Conversation,
    input: Input,
    status: Status,
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            conversation: Conversation::new(),
            input: Input::new(),
            status: Status::new(),
            state: AppState::Idle,
        };
        app.update_components_state();
        app
    }

    fn update_components_state(&mut self) {
        self.input.state = self.state;
        self.status.state = self.state;
    }

    pub fn set_state(&mut self, state: AppState) {
        self.state = state;
        self.update_components_state();
    }

    pub fn submit(&mut self) {
        self.conversation
            .conversation
            .push(Message::User(self.input.input.clone()));
        self.input.input.clear();
        self.set_state(AppState::Busy);
    }

    pub fn process_response(&mut self) {
        self.conversation.conversation.push(Message::Model(
            "I'm a Ratatui-based Gemini CLI replica. How can I help you?".to_string(),
        ));
        self.conversation.conversation.push(Message::ToolCode(
            "print(default_api.list_directory(path = \"_\"))".to_string(),
        ));
        self.set_state(AppState::Idle);
    }

    pub fn update(&mut self) -> Result<(), std::io::Error> {
        match self.state {
            AppState::Busy => {
                self.process_response();
            }
            AppState::Idle => {
                if event::poll(std::time::Duration::from_millis(250))? {
                    if let Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char(c) => self.input.input.push(c),
                                KeyCode::Backspace => {
                                    self.input.input.pop();
                                }
                                KeyCode::Enter => self.submit(),
                                KeyCode::Esc => self.set_state(AppState::Quitting),
                                _ => {}
                            }
                        }
                    }
                }
            }
            AppState::Quitting => {}
        }
        Ok(())
    }

    pub fn view(&self, f: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(0),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(f.area());

        self.conversation.view(f, layout[0]);
        self.input.view(f, layout[1]);
        self.status.view(f, layout[2]);
    }

    pub fn is_quit(&self) -> bool {
        self.state == AppState::Quitting
    }
}