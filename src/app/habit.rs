use std::{ops::Add, vec};

use chrono::{Date, Datelike, Duration, NaiveDate, TimeZone, Utc, Weekday};

type D = Date<Utc>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Habit {
    pub label: String,
    pub done_dates: Vec<chrono::Date<Utc>>,
}

impl Habit {
    fn random() -> Self {
        Habit {
            label: "random".into(),
            done_dates: vec![],
        }
    }

    pub fn check_task(&mut self, date: D) {
        match self.done_dates.iter().position(|&x| x == date) {
            Some(i) => {
                self.done_dates.remove(i);
            }
            None => self.done_dates.push(date),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HabitTracker {
    pub start_date: D,
    pub habits: Vec<Habit>,
}

impl HabitTracker {
    pub fn next_week(&mut self) {
        let start_date = self.start_date;
        self.start_date = start_date.add(Duration::days(7));
    }

    pub fn previous_week(&mut self) {
        let start_date = self.start_date;
        self.start_date = start_date.checked_sub_signed(Duration::days(7)).unwrap();
    }

    pub fn get_date_range(&self) -> Vec<Date<Utc>> {
        let dates: &mut Vec<D> = &mut vec![];
        dates.push(self.start_date);
        for i in 1..7 {
            dates.push(self.start_date.add(Duration::days(i)));
        }
        dates.to_owned()
    }

    pub fn get_header_labels(&self) -> Vec<String> {
        self.get_date_range()
            .iter()
            .map(|d| d.day().to_string())
            .collect()
    }

    // This should send a matrix of bools, which should contain
    pub fn values(&self) -> Vec<Vec<bool>> {
        let date_range = self.get_date_range();
        let mut values: Vec<Vec<bool>> = vec![vec![false; date_range.len()]; self.habits.len()];
        for (i, habit) in self.habits.iter().enumerate() {
            for (j, date) in date_range.iter().enumerate() {
                if habit.done_dates.contains(&date) {
                    values[i][j] = true;
                }
            }
        }
        values
    }

    pub fn labels(&self) -> Vec<String> {
        let mut labels: Vec<String> = vec![];
        for habit in self.habits.clone() {
            labels.push(habit.label)
        }
        labels
    }

    fn week_bounds(week: u32) -> Date<Utc> {
        let offset = chrono::offset::Local::now();
        let current_year = offset.year();
        let mon = NaiveDate::from_isoywd(current_year, week, Weekday::Mon);
        let sun = NaiveDate::from_isoywd(current_year, week, Weekday::Sun);
        Date::<Utc>::from_utc(mon, Utc)
    }

    pub fn random() -> Self {
        HabitTracker {
            start_date: HabitTracker::week_bounds(Utc::now().iso_week().week()),
            habits: vec![Habit::random(); 5],
        }
    }
}
