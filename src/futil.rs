use std::fs::File;
use std::io::{self, BufReader, BufRead, Write};
use std::path::Path;
use std::fs::OpenOptions;
use crate::entry::Entry;

const TASK_FILE_PATH: &str = "tasks.json";

pub fn write_file(value:&str) -> Result<(), String> {
    let mut task_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .open(TASK_FILE_PATH)
        {
            Ok(file) => file,
            Err(e) => return Err(e.to_string())
        };
        task_file
        .write(value.as_bytes())
        .expect("Failed writing to file");

    Ok(())
}

pub fn get_last_entry() -> Result<String,String> {
    if let Ok(lines) = read_lines(TASK_FILE_PATH) {
        if let Some(last) = lines.map_while(Result::ok).last() {
            Ok(last)
        }
        else {
            Ok(String::new())
        }
    }
    else {
        return Err("Can't open file".to_string());
    }
}

pub fn get_all_entries() -> Result<Vec<Entry>, String> {
    let file = match OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(TASK_FILE_PATH)
        {
            Ok(f) => f,
            Err(e) => return Err(e.to_string())
        };

    let reader = BufReader::new(file);

    let entries = match serde_json::from_reader(reader) {
        Ok(e) => e,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => return Err(e.to_string())
    };

    Ok(entries)
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let task_file = match OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(filename)
        {
            Ok(file) => file,
            Err(e) => {
                println!("Failed to open file: {}", e);
                std::process::exit(0);
            }
        };

    Ok(io::BufReader::new(task_file).lines())
}