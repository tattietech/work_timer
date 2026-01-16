use std::env;
use crate::{tracker::{start, stop, print_all, print_last}};

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
        "all" => print_all(),
        "last" => print_last(),
        _ => Err("Useage: task <start|stop> <name>\n\n Mode must be 'start' or 'stop'".to_string())
    };

    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(0);
    }

    Ok(())
}