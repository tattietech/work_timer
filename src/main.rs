use std::env;
use crate::tracker::{start, stop, view_all};

mod futil;
mod tracker;
mod timeutil;

fn main() -> Result<(), String> {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Useage: task <start|stop> <name>\n\n Or task <stop>");
        std::process::exit(0);
    }
    
    let mode = &args[1];
    let name = if args.len() >=3 {&args[2..].join(" ")} else {""};

    let result = match &mode[..] {
        "start" => start(name),
        "stop" => stop(),
        "view" => view_all(),
        _ => Err("Useage: task <start|stop> <name>\n\n Mode must be 'start' or 'stop'".to_string())
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(0);
    }

    Ok(())
}