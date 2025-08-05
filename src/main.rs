mod components;
mod model;
use components::app::App;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{self, stdout, Stdout};

fn main() -> io::Result<()> {
    // The app is started by calling the run function.
    let mut terminal = match init_terminal() {
        Ok(terminal) => terminal,
        Err(e) => {
            eprintln!("Failed to initialize terminal: {}", e);
            return Err(e);
        }
    };
    let mut app = App::new();

    // The main loop of the application.
    while !app.is_quit() {
        // The view function is called to render the UI.
        terminal.draw(|frame| app.view(frame))?;

        // The update function is called to handle user input.
        app.update()?;
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
    Ok(())
}