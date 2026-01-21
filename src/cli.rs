use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "task",
    about = "Task Time Keeper",
    long_about = None
)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start a task
    Start {
        /// Name of the tasks (supports spaces if quoted)
        name: String,
    },

    /// Stop the current task
    Stop,

    /// View all tasks logged
    All,

    /// View the active or last logged task
    Last,
}