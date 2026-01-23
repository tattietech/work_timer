use clap::Parser;

use crate::{cli::{Cli, Commands}, tracker::{start, stop, print_last, print_all}};

mod futil;
mod tracker;
mod cli;
mod entry;

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Start {name} => start(&name)?,
        Commands::Stop => stop()?,
        Commands::All => print_all()?,
        Commands::Last => print_last()?
    };

    Ok(())
}