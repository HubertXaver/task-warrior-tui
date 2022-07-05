use std::default;
use std::fmt::Display;
use std::time::Duration;

use chrono::prelude::*;
use chrono::Utc;
use derivative::Derivative;

#[derive(Derivative, Debug, PartialEq, Eq)]
#[derivative(Default)]
enum PomodoroStatus {
    Running,
    Paused,
    #[derivative(Default)]
    Stopped,
}

#[derive(Debug, Default)]
struct Pomodoro {
    pub status: PomodoroStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub paused_seconds: i64,
    pub paused_at: Option<DateTime<Utc>>,
}

/**
 * Start a pomodoro instance
 * Poll the object to check the time elapsed
 * Stop or Pause
 * No Tokio crap
 */

impl Pomodoro {
    /**
     * Cases
     * 1. Elapsed time when the timer has never been paused
     * 2. Elapsed time when the timer has been paused multiple times and resumed
     * 3. Elapsed time when the timer has been paused for now
     */
    pub fn elapsed_time(&self) -> Result<chrono::Duration, String> {
        if self.started_at == None || self.status == PomodoroStatus::Stopped {
            return Err("Error getting elapsed time".to_owned());
        }
        // Time calculation
        let now = match self.status {
            PomodoroStatus::Paused => self.paused_at.unwrap(),
            _ => Utc::now(),
        };
        let difference = (now - self.started_at.unwrap())
            .checked_sub(&chrono::Duration::seconds(self.paused_seconds))
            .unwrap();

        Ok(difference)
    }

    pub fn pause(&mut self) {
        println!("Pomodoro Paused");
        self.status = PomodoroStatus::Paused;
        self.paused_at = Some(Utc::now());
    }

    pub fn start(&mut self) {
        match self.status {
            PomodoroStatus::Paused => {
                self.paused_seconds +=
                    (Utc::now() - self.paused_at.unwrap_or(Utc::now())).num_seconds()
            }
            PomodoroStatus::Stopped => self.started_at = Some(Utc::now()),
            _ => {}
        };
        println!("Pomodoro Started");
        self.status = PomodoroStatus::Running;
    }

    pub fn stop(&mut self) {
        println!("Pomodoro Stopped");
        self.status = PomodoroStatus::Stopped;
    }
}

impl Display for Pomodoro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Status: {:?}, Started At: {:?}, Pause Seconds: {:?}, Elapsed Time {:?}",
            self.status,
            self.started_at,
            self.paused_seconds,
            self.elapsed_time()
                .unwrap_or(chrono::Duration::seconds(0))
                .num_seconds()
        )
    }
}

async fn sleep(n: u64) {
    tokio::time::sleep(std::time::Duration::from_secs(n)).await;
}

#[cfg(test)]
mod test {

    use super::{sleep, Pomodoro};

    #[tokio::test]
    async fn test() {
        let p = &mut Pomodoro::default();
        p.start();
        println!("{}", p); // 0
        sleep(3).await;
        p.pause();
        println!("{}", p); // 3
        sleep(3).await;
        p.start();
        println!("{}", p); // 3
        sleep(2).await;
        p.pause();
        println!("{}", p); // 5
        sleep(2).await;
        p.start();
        println!("{}", p); // 5
        sleep(3).await;
        p.pause();
        sleep(2).await;
        println!("{}", p); // 5
        p.stop();
    }
}
