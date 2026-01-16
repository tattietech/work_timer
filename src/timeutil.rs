use chrono::prelude::*;
use chrono::TimeDelta;

pub fn local_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn time_between(from:&str, to:&str) -> Result<String,String> {
    let from_date = match NaiveDateTime::parse_from_str(from, "%Y-%m-%d %H:%M:%S") {
        Ok(d) => d,
        Err(e) => return Err(format!("Could not parse from date: {e}"))
    };

    let to_date = match NaiveDateTime::parse_from_str(to, "%Y-%m-%d %H:%M:%S") {
        Ok(d) => d,
        Err(e) => return Err(format!("Could not parse from date: {e}"))
    };

    let difference = to_date - from_date;

    let hours = difference.num_hours();
    let minutes = (difference - TimeDelta::hours(hours)).num_minutes();
    let seconds = (difference - TimeDelta::hours(hours) - TimeDelta::minutes(minutes)).num_seconds();
    
    Ok(format!("{:0>2}:{:0>2}:{:0>2}", hours,minutes,seconds))
}