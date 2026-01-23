use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use tabled::{Tabled};

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Entry {
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub name: String,
    pub active: bool,
    pub duration: String
}

impl Entry {
    pub fn set_duration(&mut self) {
            let dur = self.end_time - self.start_time;
            let dur_secs = dur.num_seconds();

            let hours = dur_secs / 3600;
            let minutes = (dur_secs % 3600) / 60;
            let seconds = dur_secs % 60;

            self.duration = format!("{hours:02}:{minutes:02}:{seconds:02}");
        }
    }