use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::fs::OpenOptions;
use chrono::prelude::*;
use chrono::TimeDelta;

const TASK_FILE_PATH: &str = "tasks.csv";

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Useage: task <start|stop> <name>\n\n Or task <stop>");
        std::process::exit(0);
    }
    
    let mode = &args[1];
    let name = if args.len() >=3 {&args[2..].join(" ")} else {""};

    match &mode[..] {
        "start" => start(name),
        "stop" => stop(),
        _ => {
            println!("Useage: task <start|stop> <name>\n\n Mode must be 'start' or 'stop'");
            std::process::exit(0);
        }
    }
}

fn start(name:&str) {
    if name.len() == 0 {
        println!("Please enter name of task.");
        std::process::exit(0);
    }

    let last_entry = get_last_entry();
    let last_char = last_entry
    .chars()
    .last()
    .unwrap_or(' ');

    // if the file is not empty and the last entry is still open
    if last_entry != "" && last_char == ',' {
        println!("An entry is still open, please run task stop to stop it.");
        std::process::exit(0);
    }

    println!("Starting task: {name}");
    
    let local_time = local_time();       
    let line = format!("{name},{local_time}");
    append_to_file(&line);
}

fn stop() {
    let last_entry = get_last_entry();
    let last_arr: Vec<&str> = last_entry.split(',').collect();

    if last_arr.len() > 2 {
        println!("No task running.");
        std::process::exit(0);
    }

    let last_time = last_arr.last().unwrap();
    let local_time = local_time();

    let diff = time_between(&last_time, &local_time).to_string();

    let line = format!(",{local_time},{diff}\n");

    append_to_file(&line);

    let last_name = last_arr.first().unwrap();
    println!("Task {last_name} stopped at {diff}");
}

fn append_to_file(value:&str) {
    let mut task_file = match OpenOptions::new()
        .append(true)
        .create(true)
        .open(TASK_FILE_PATH)
        {
            Ok(file) => file,
            Err(e) => {
                println!("Failed to open file: {}", e);
                std::process::exit(0);
            }
        };
        task_file
        .write(value.as_bytes())
        .expect("Failed writing to file");
}

fn local_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn get_last_entry() -> String {
    if let Ok(lines) = read_lines(TASK_FILE_PATH) {
        if let Some(last) = lines.map_while(Result::ok).last() {
            last
        }
        else {
            String::new()
        }
    }
    else {
        println!("Can't open file");
        std::process::exit(0);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
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

fn time_between(from:&str, to:&str) -> String {
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