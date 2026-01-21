use chrono::Local;

use crate::futil::{append_to_file, get_last_entry, get_all};
use crate::timeutil::{local_time, time_between};
use crate::entry::Entry;

pub fn start(name:&str) -> Result<(),String> {
    if name.len() == 0 {
        return Err("Please enter name of task.".to_string());
    }

    let last_entry = get_last_entry()?;

    let last_char = last_entry
    .chars()
    .last()
    .unwrap_or(' ');

    // if the file is not empty and the last entry is still open
    if last_entry != "" && last_char == ',' {
        return Err("An entry is still open, please run task stop to stop it.".to_string());
    }

    let entry = Entry {
        start_time: Local::now(),
        end_time: None,
        name: name.to_string(),
        active: true
    };

    println!("Starting task: {name}");
      
    let line = format!("{name},{}", local_time());
    append_to_file(&line)?;

    Ok(())
}

pub fn stop() -> Result<(), String> {
    let last_entry = get_last_entry()?;

    let last_arr: Vec<&str> = last_entry.split(',').collect();

    if last_arr.len() > 2 {
        return Err("No task is running".to_string());
    }

    let last_time = last_arr.last().unwrap();
    let local_time = local_time();

    let diff = time_between(&last_time, &local_time)?.to_string();

    let line = format!(",{local_time},{diff}\n");

    append_to_file(&line)?;

    let last_name = last_arr.first().unwrap();
    println!("Task {last_name} stopped at {diff}");

    Ok(())
}

pub fn print_all() -> Result<(), String> {
    let entries = get_all()?;

    if entries.is_empty() {
        println!("No entries found");
        return Ok(());
    }

    for line in entries {
        println!("{}",line);
    }

    Ok(())
}

pub fn print_last() -> Result<(), String> {
    let last = get_last_entry()?;

    println!("{last}");

    Ok(())
}