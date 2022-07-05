use derive_new::new;
use eyre::Result;
use std::process::Command;

use tui::widgets::ListState;

pub mod pomodoro;
pub mod ui;

pub enum InputMode {
    Normal,
    Editing,
    Pomodoro,
}
pub struct App {
    pub input: String,
    pub mode: InputMode,
    pub messages: Vec<String>,
    pub state: ListState,
}

impl App {
    pub fn new() -> Result<App> {
        let mut app = App {
            input: String::new(),
            mode: InputMode::Normal,
            messages: Vec::new(),
            state: ListState::default(),
        };
        app.get_tasks()?;
        app.next_task();
        return Ok(app);
    }

    pub fn next_task(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.messages.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous_task(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.messages.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
    pub fn add_task(&mut self, task: String) -> Result<()> {
        let result = Command::new("task").arg("add").arg(task).output()?;
        println!("{:?}", result.stderr);
        Ok(())
    }

    pub fn get_tasks(&mut self) -> Result<()> {
        let result = Command::new("task").output().unwrap();
        let output = String::from_utf8_lossy(&result.stdout)
            .to_string()
            .split("\n")
            .enumerate()
            .filter(|(i, _)| match *i {
                0 => false,
                1 => false,
                2 => false,
                _ => true,
            })
            .map(|(_, t)| t.to_string())
            .into_iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>();
        self.messages = output;
        Ok(())
    }

    pub fn start_task(&mut self) -> Result<()> {
        self.mode = InputMode::Pomodoro;
        Ok(())
    }
}
