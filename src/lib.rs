use crate::app::ui;
use app::{App, AppMode};
use crossterm::event::{self, Event, KeyCode};
use eyre::Result;
use std::{cell::RefCell, io::stdout, rc::Rc};

use tui::{backend::CrosstermBackend, Terminal};

pub mod app;

pub fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        let mut app = app.borrow_mut();
        terminal.draw(|rect| ui::draw(rect, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                app::AppMode::NORMAL => match key.code {
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('k') => app.up(),
                    KeyCode::Char('j') => app.down(),
                    KeyCode::Char('h') => app.left(),
                    KeyCode::Char('l') => app.right(),
                    KeyCode::Char(' ') => app.check(),
                    KeyCode::Char(':') => app.enter_command_mode(),
                    _ => {}
                },
                app::AppMode::COMMAND => match key.code {
                    KeyCode::Esc => {
                        app.input = String::new();
                        app.mode = AppMode::NORMAL;
                    }
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Enter => {
                        app.execute_input();
                        app.mode = AppMode::NORMAL
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    _ => {}
                },
            }
        }
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
