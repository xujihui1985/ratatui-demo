use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{self, stdout, Stdout};

mod components;
mod model;
mod update;
mod view;

use model::{AppState, Model};

fn main() -> io::Result<()> {
    // The app is started by calling the run function.
    let mut terminal = init_terminal()?;
    let mut model = Model::new();

    // The main loop of the application.
    while model.state != AppState::Quitting {
        // The view function is called to render the UI.
        terminal.draw(|frame| view::view(&model, frame))?;

        // The update function is called to handle user input.
        update::update(&mut model)?;
    }

    // The app is terminated by calling the restore_terminal function.
    restore_terminal()
}

/// Initializes the terminal.
fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    crossterm::execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restores the terminal to its original state.
fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())}