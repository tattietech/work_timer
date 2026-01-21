use chrono::{DateTime, Local};

pub struct Entry {
    pub start_time: DateTime<Local>,
    pub end_time: Option<DateTime<Local>>,
    pub name: String,
    pub active: bool
}

impl Entry {
    fn duration_in_secs(&self) -> i64 {
        if let Some(et) = self.end_time {
            let dur = et - self.start_time;
            dur.num_seconds()
        }
        else {
            0
        }
    }
}