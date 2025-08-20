//! The main executable for the command-line task manager application.
//!
//! This file initializes the application, parses command-line arguments,
//! and dispatches to the appropriate functions for task management.

use clap::Parser;
use log::{debug, error, info};
use task_manager_command_line::app::storage;
use task_manager_command_line::{AppError, Cli, Commands, Result, Task};

fn main() -> Result<()> {
    env_logger::init();
    info!("Task manager application started.");

    let cli = Cli::parse();
    debug!("Parsed CLI command: {:?}", cli.command);

    let mut tasks = storage::load_tasks()?;
    info!("Loaded {} tasks from storage.", tasks.len());

    match cli.command {
        Commands::Add { description, due } => {
            handle_add_task(&mut tasks, description, due)?;
        }
        Commands::List { all } => {
            handle_list_tasks(&tasks, all);
        }
        Commands::Complete { id } => {
            handle_mark_task_completion(&mut tasks, id, true)?;
        }
        Commands::Undone { id } => {
            handle_mark_task_completion(&mut tasks, id, false)?;
        }
        Commands::Remove { id } => {
            handle_remove_task(&mut tasks, id)?;
        }
        Commands::Clear { yes } => {
            handle_clear_tasks(&mut tasks, yes)?;
        }
    }

    storage::save_tasks(&tasks)?;
    info!("Tasks saved to storage. Application finished.");

    Ok(())
}

/// Handles the 'add' command.
/// Generates a new unique ID for the task and adds it to the list.
fn handle_add_task(
    tasks: &mut Vec<Task>,
    description: String,
    due_date: Option<chrono::NaiveDate>,
) -> Result<()> {
    let new_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    let new_task = Task::new(new_id, description, due_date);
    info!("Adding new task: {:?}", new_task);
    tasks.push(new_task);

    println!("Task added: ID {}", new_id);
    Ok(())
}

/// Handles the 'list' command.
/// Prints tasks to the console, optionally including completed ones.
fn handle_list_tasks(tasks: &[Task], show_all: bool) {
    if tasks.is_empty() {
        println!("No tasks found. Add one with `task add <description>`");
        return;
    }

    println!("ID   Description                  Due Date    Status");
    println!("---- ---------------------------- ----------- --------");

    let mut found_tasks = false;
    for task in tasks {
        if show_all || !task.completed {
            let status = if task.completed { "DONE" } else { "PENDING" };
            let due_date_str = task
                .due_date
                .map_or("N/A".to_string(), |d| d.format("%Y-%m-%d").to_string());
            println!(
                "{:<4} {:<28} {:<11} {}",
                task.id, task.description, due_date_str, status
            );
            found_tasks = true;
        }
    }

    if !found_tasks && !show_all {
        println!("All tasks completed! Good job. Use `list --all` to see them.");
    }
}

/// Handles marking a task as complete or incomplete.
fn handle_mark_task_completion(tasks: &mut Vec<Task>, id: u32, status: bool) -> Result<()> {
    let task_found = tasks.iter_mut().find(|t| t.id == id);

    match task_found {
        Some(task) => {
            task.mark_completion(status);
            println!(
                "Task ID {} marked as {}",
                id,
                if status { "completed" } else { "incomplete" }
            );
            Ok(())
        }
        None => {
            error!(
                "Attempted to mark completion for non-existent task ID: {}",
                id
            );
            Err(AppError::TaskNotFound(id))
        }
    }
}

/// Handles the 'remove' command.
fn handle_remove_task(tasks: &mut Vec<Task>, id: u32) -> Result<()> {
    let initial_len = tasks.len();
    tasks.retain(|task| task.id != id);

    if tasks.len() < initial_len {
        println!("Task ID {} removed.", id);
        Ok(())
    } else {
        error!("Attempted to remove non-existent task ID: {}", id);
        Err(AppError::TaskNotFound(id))
    }
}

/// Handles the 'clear' command.
/// Clears all tasks after confirmation.
fn handle_clear_tasks(tasks: &mut Vec<Task>, confirmed: bool) -> Result<()> {
    if !confirmed {
        println!("This will remove ALL tasks. Use `task clear --yes` to confirm.");
        return Ok(());
    }

    let num_cleared = tasks.len();
    tasks.clear();
    println!("Cleared {} tasks.", num_cleared);
    Ok(())
}
