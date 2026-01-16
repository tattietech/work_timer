use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::fs::OpenOptions;

const TASK_FILE_PATH: &str = "tasks.csv";

pub fn append_to_file(value:&str) -> Result<(), String> {
    let mut task_file = match OpenOptions::new()
        .append(true)
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

pub fn get_all() -> Result<Vec<String>, String> {
    let lines = read_lines(TASK_FILE_PATH)
    .map_err(|e| format!("Can't read file: {}",e))?;

    let mut out = Vec::new();

    for line in lines {
        match line {
            Ok(l) => out.push(l),
            Err(e) => return Err(format!("Error reading line: {e}")),
        }
    }

    Ok(out)
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