use crate::futil::{append_to_file, get_last_entry};
use crate::timeutil::{local_time, time_between};

pub fn start(name:&str) {
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

pub fn stop() {
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