use clap::Parser;

use crate::{cli::{Cli, Commands}, tracker::{start, stop, print_last, view_all}};

mod futil;
mod tracker;
mod cli;
mod entry;

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Start {name} => start(&name)?,
        Commands::Stop => stop()?,
        Commands::All => view_all()?,
        Commands::Last => print_last()?
    };

    Ok(())
}