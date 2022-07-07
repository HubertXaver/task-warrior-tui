use commands::tokenizer::{tokenize, TokenType};
use eyre::Result;

use self::habit::{Habit, HabitTracker};

pub mod pomodoro;
pub mod ui;

pub mod habit;

#[derive(Default, Debug, Clone)]
pub struct AppState {
    selected_row: Option<usize>,
    selected_column: Option<usize>,
}

impl AppState {
    pub fn selected(&self) -> Option<(usize, usize)> {
        if self.selected_row == None || self.selected_column == None {
            return None;
        }
        Some((self.selected_row.unwrap(), self.selected_column.unwrap()))
    }

    pub fn select(&mut self, (row, col): (usize, usize)) {
        self.selected_column = Some(col);
        self.selected_row = Some(row)
    }
}

#[derive(Debug, Clone)]
pub enum AppMode {
    NORMAL,
    COMMAND,
}

#[derive(Debug, Clone)]
pub struct App {
    pub mode: AppMode,
    pub state: AppState,
    pub tracker: HabitTracker,
    pub input: String,
}

impl App {
    pub fn new() -> Result<App> {
        let app = App {
            state: AppState::default(),
            tracker: HabitTracker::random(),
            mode: AppMode::NORMAL,
            input: String::new(),
        };
        return Ok(app);
    }

    pub fn enter_command_mode(&mut self) {
        self.input = String::new();
        self.mode = AppMode::COMMAND;
    }

    pub fn execute_input(&mut self) {
        let input = self.input.as_str();
        if let Ok(tokens) = tokenize(input) {
            if tokens[0].text == "add".to_owned() {
                if tokens.len() != 3 && tokens[2].token_type != TokenType::Whitespace {
                    self.input = "[1] Error! please use format `add 'habit name'`".to_owned();
                    return;
                } else {
                    self.add_habit(tokens[2].text.to_owned());
                    return;
                }
            } else {
                self.input = "[3] Error! please use format `add 'habit name'`".to_owned();
                return;
            }
        } else {
            self.input = "[2] Error! please use format `add 'habit name'`".to_owned();
        }
    }

    pub fn add_habit(&mut self, habit: String) {
        self.tracker.habits.push(Habit {
            label: habit,
            done_dates: vec![],
        })
    }
    pub fn check(&mut self) {
        if self.state.selected().is_none() {
            return;
        }
        let (row, col) = self.state.selected().unwrap();
        let date = self.tracker.get_date_range()[col];
        self.tracker.habits[row].check_task(date);
    }

    pub fn down(&mut self) {
        let i = match self.state.selected() {
            Some((row, col)) => {
                if row == self.tracker.values().len() - 1 {
                    (0, col)
                } else {
                    (row + 1, col)
                }
            }
            None => (0, 0),
        };
        self.state.select(i)
    }

    pub fn up(&mut self) {
        let i = match self.state.selected() {
            Some((row, col)) => {
                if row == 0 {
                    (self.tracker.values().len() - 1, col)
                } else {
                    (row - 1, col)
                }
            }
            None => (0, 0),
        };
        self.state.select(i)
    }

    pub fn left(&mut self) {
        let i = match self.state.selected() {
            Some((row, col)) => {
                if col == 0 {
                    self.tracker.previous_week();
                    (row, self.tracker.values()[0].len() - 1)
                } else {
                    (row, col - 1)
                }
            }
            None => (0, 0),
        };
        self.state.select(i)
    }

    pub fn right(&mut self) {
        let i = match self.state.selected() {
            Some((row, col)) => {
                let length = self.tracker.values()[0].len();
                if col == length - 1 {
                    self.tracker.next_week();
                    (row, 0)
                } else {
                    (row, col + 1)
                }
            }
            None => (0, 0),
        };
        self.state.select(i)
    }
}
