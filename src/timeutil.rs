use chrono::prelude::*;
use chrono::TimeDelta;

pub fn local_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn time_between(from:&str, to:&str) -> String {
    let from_date = match NaiveDateTime::parse_from_str(from, "%Y-%m-%d %H:%M:%S") {
        Ok(d) => d,
        Err(e) => {
                println!("Could not parse from date: {}", e);
                std::process::exit(0);
            }
    };

    let to_date = match NaiveDateTime::parse_from_str(to, "%Y-%m-%d %H:%M:%S") {
        Ok(d) => d,
        Err(e) => {
                println!("Could not parse to date: {}", e);
                std::process::exit(0);
            }
    };

    let difference = to_date - from_date;

    let hours = difference.num_hours();
    let minutes = (difference - TimeDelta::hours(hours)).num_minutes();
    let seconds = (difference - TimeDelta::hours(hours) - TimeDelta::minutes(minutes)).num_seconds();
    
    format!("{:0>2}:{:0>2}:{:0>2}", hours,minutes,seconds)
}