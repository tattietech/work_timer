use std::env;
use crate::tracker::{start, stop, view_all};

mod futil;
mod tracker;
mod timeutil;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Useage: task <start|stop> <name>\n\n Or task <stop>");
        std::process::exit(0);
    }
    
    let mode = &args[1];
    let name = if args.len() >=3 {&args[2..].join(" ")} else {""};

    fn view() {
        view_all();
    }

    match &mode[..] {
        "start" => start(name),
        "stop" => stop(),
        "view" => view(),
        _ => {
            println!("Useage: task <start|stop> <name>\n\n Mode must be 'start' or 'stop'");
            std::process::exit(0);
        }
    }
}