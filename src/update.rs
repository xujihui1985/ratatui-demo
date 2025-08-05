use crate::model::{AppState, Model};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

/// The update function handles application logic and state changes.
pub fn update(model: &mut Model) -> Result<(), std::io::Error> {
    match model.state {
        AppState::Busy => {
            model.process_response();
        }
        AppState::Idle => {
            if event::poll(std::time::Duration::from_millis(250))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char(c) => model.input.input.push(c),
                            KeyCode::Backspace => {
                                model.input.input.pop();
                            }
                            KeyCode::Enter => model.submit(),
                            KeyCode::Esc => model.set_state(AppState::Quitting),
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
