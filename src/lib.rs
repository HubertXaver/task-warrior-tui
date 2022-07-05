use crate::app::ui;
use app::{App, InputMode};
use crossterm::event::{self, Event, KeyCode};
use eyre::Result;
use std::{cell::RefCell, io::stdout, rc::Rc};

use tui::{backend::CrosstermBackend, Terminal};

pub mod app;
pub mod inputs;

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
                app::InputMode::Normal => match key.code {
                    KeyCode::Char('e') | KeyCode::Char('i') => {
                        app.mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        break;
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        app.next_task();
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        app.previous_task();
                    }
                    KeyCode::Enter => {

                    }
                    _ => {}
                },
                app::InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        let input: String = app.input.drain(..).collect();
                        app.add_task(input)?;
                        app.get_tasks()?;
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Esc => {
                        app.mode = InputMode::Normal;
                    }
                    _ => {}
                },
                app::InputMode::Pomodoro => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.mode = InputMode::Normal;
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
