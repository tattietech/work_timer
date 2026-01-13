use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Useage: task <start|stop> <name>\nMust provide two arguments.");
        std::process::exit(1);
    }
    
    let mode = &args[1];
    let name = &args[2];

    match &mode[..] {
        "start" => start(name),
        "stop" => stop(name),
        _ => {
            println!("Useage: task <start|stop> <name>\n\n Mode must be 'start' or 'stop'");
            std::process::exit(1);
        }
    }
}

fn start(name:&str) {
    println!("Starting task: {name}");
}

fn stop(name:&str) {
    println!("Stopping task: {name}");
}
