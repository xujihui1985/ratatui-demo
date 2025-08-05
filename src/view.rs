use crate::model::Model;
use ratatui::prelude::{Constraint, Direction, Frame, Layout};

/// Renders the user interface widgets.
pub fn view(model: &Model, f: &mut Frame) {
    // The layout defines the arrangement of widgets in the terminal.
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(f.area());

    model.conversation.view(f, layout[0]);
    model.input.view(f, layout[1]);
    model.status.view(f, layout[2]);
}
