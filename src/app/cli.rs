//! Defines the command-line interface (CLI) structure using `clap`.
//!
//! This module specifies the application's commands, subcommands, and arguments,
//! allowing `clap` to parse user input from the terminal.

use chrono::NaiveDate;
use clap::{Parser, Subcommand};

/// This struct uses `clap`'s `Parser` trait to automatically parse command-line arguments.
#[derive(Parser, Debug)]
#[command(
    author = "0xgsvs",
    version = "0.1.0",
    about = "A simple command-line task manager written in Rust.",
    long_about = "Organize your tasks efficiently from the terminal. Add, list, complete, and remove tasks with ease."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Defines the available commands for the task manager.
///
/// Each variant represents a distinct action (e.g., adding a task, listing tasks).
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new task.
    ///
    /// The task description is required. An optional due date can be specified.
    Add {
        /// The description of the task to add.
        description: String,
        /// Optional due date for the task (format: YYYY-MM-DD).
        #[arg(short, long, value_parser = parse_due_date)]
        due: Option<NaiveDate>,
    },
    /// List all tasks.
    ///
    /// By default, only incomplete tasks are shown. Use the --all flag to see all tasks.
    List {
        /// Show all tasks, including completed ones.
        #[arg(short, long)]
        all: bool,
    },
    /// Mark a task as complete.
    ///
    /// Requires the ID of the task to mark.
    Complete {
        /// The ID of the task to mark as complete.
        id: u32,
    },
    /// Mark a task as incomplete.
    ///
    /// Requires the ID of the task to mark.
    Undone {
        /// The ID of the task to mark as incomplete.
        id: u32,
    },
    /// Remove a task.
    ///
    /// Requires the ID of the task to remove.
    Remove {
        /// The ID of the task to remove.
        id: u32,
    },
    /// Remove all tasks.
    ///
    /// Requires confirmation to prevent accidental data loss.
    Clear {
        /// Confirm removal of all tasks.
        #[arg(short, long)]
        yes: bool,
    },
}

/// Helper function to parse a string into a `NaiveDate`.
///
/// Used by `clap`'s `value_parser` to validate and convert the `due` argument.
/// Returns a `Result` indicating success or failure of parsing.
fn parse_due_date(s: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| format!("Date format must be YYYY-MM-DD. Failed to parse: '{}'", s))
}
