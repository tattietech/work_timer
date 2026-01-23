use chrono::Local;
use tabled::{Table};
use crate::futil::{get_last_entry, get_all_entries, write_file};
use crate::entry::Entry;
use serde_json;

pub fn start(name:&str) -> Result<(),String> {
    if name.len() == 0 {
        return Err("Please enter name of task.".to_string());
    }

    let mut entries : Vec<Entry> = get_all_entries()?;

    if entries.len() > 0 {
        let last_entry = entries.last().unwrap();

        if last_entry.active {
            return Err("An entry is still open, please run task stop to stop it.".to_string());   
        }
    }

    let entry = Entry {
        start_time: Local::now(),
        end_time: Local::now(),
        name: name.to_string(),
        active: true,
        duration: "0".to_string()
    };

    entries.push(entry);

    let ser_entries = serde_json::to_string(&entries).unwrap();

    write_file(&ser_entries)?;

    Ok(())
}

pub fn stop() -> Result<(), String> {
    let mut entries : Vec<Entry> = get_all_entries()?;

    if let Some(last) = entries.last_mut() {
        if !last.active {
                return Err("No task is running".to_string());
            }

        last.end_time = Local::now();
        last.set_duration();
        last.active = false;

        let ser_entries = serde_json::to_string(&entries).unwrap();
        write_file(&ser_entries)?;
    }
    else {
        return Err("Problem opening file.".to_string());
    }

    Ok(())
}

pub fn print_all() -> Result<(), String> {
    let entries = get_all_entries().unwrap();

    let table = Table::new(entries);

    println!("{}", table);

    Ok(())
}

pub fn print_last() -> Result<(), String> {
    let last = get_last_entry()?;

    println!("{last}");

    Ok(())
}