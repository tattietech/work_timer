# Task Time Keeper

A simple command-line task time tracking tool written in Rust. This application helps you track how much time you spend on different tasks throughout your day.

## Features

- Start and stop task timers
- Automatic duration calculation
- View all logged tasks
- View the last or currently active task
- Persistent storage in JSON format
- Clean table-formatted output

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/tattietech/work_timer.git
cd work_timer
```

2. Build the project:
```bash
cargo build --release
```

3. The binary will be available at `target/release/task`

### Optional: Install Globally

To install the binary globally on your system:
```bash
cargo install --path .
```

## Usage

The application provides four main commands:

### Start a Task

Start tracking a new task with a given name:
```bash
task start "Task name"
```

Example:
```bash
task start "Writing documentation"
```

Note: You can only have one active task at a time. Stop the current task before starting a new one.

### Stop a Task

Stop the currently running task:
```bash
task stop
```

This will record the end time and calculate the total duration.

### View All Tasks

Display all logged tasks in a table format:
```bash
task all
```

This shows:
- Task name
- Start time
- End time
- Active status
- Duration

### View Last Task

Display only the last logged task or the currently active task:
```bash
task last
```

## Data Storage

Tasks are stored in a `tasks.json` file in the current working directory. The file contains:
- Task name
- Start time (format: `YYYY-MM-DD HH:MM:SS`)
- End time (format: `YYYY-MM-DD HH:MM:SS`)
- Active status (true/false)
- Duration (format: `HH:MM:SS`)

## Project Structure

```
work_timer/
├── src/
│   ├── main.rs      # Application entry point
│   ├── cli.rs       # Command-line interface definitions
│   ├── tracker.rs   # Core tracking logic
│   ├── entry.rs     # Task entry data structure
│   └── futil.rs     # File utility functions
├── Cargo.toml       # Project dependencies
└── Cargo.lock       # Dependency lock file
```

## Dependencies

- **clap** (4.5.54) - Command-line argument parsing
- **chrono** (0.4) - Date and time handling
- **serde** (1.0) - Serialization framework
- **serde_json** (1.0) - JSON serialization
- **tabled** (0.15) - Table formatting for output

## Development

### Running Tests

```bash
cargo test
```

### Running in Development

```bash
cargo run -- start "My task"
cargo run -- stop
cargo run -- all
cargo run -- last
```

### Code Style

Format code using rustfmt:
```bash
cargo fmt
```

Lint code using clippy:
```bash
cargo clippy
```

## Error Handling

The application will return helpful error messages for common issues:
- Attempting to start a task when one is already active
- Attempting to stop a task when none is running
- Providing an empty task name
- File operation failures

## License

This project is open source. Please check the repository for license information.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
