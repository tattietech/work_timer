use chrono::{NaiveDateTime};
use serde::{Serialize, Deserialize};
use tabled::{Tabled};

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Entry {
    pub name: String,
    
    #[serde(with = "my_date_format")]
    pub start_time: NaiveDateTime,

    #[serde(with = "my_date_format")]
    pub end_time: NaiveDateTime,
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

mod my_date_format {
    use chrono::{NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(
        date: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;

        Ok(dt)
    }
}