//! Handles loading and saving tasks to a TOML file.
//!
//! This module manages the persistence layer of the task manager,
//! ensuring tasks are saved between application runs.

#![allow(unused_imports)]
use log::{debug, error, info};
use std::{fs, path::PathBuf};

use crate::app::{AppError, Result, Task, TaskList};

/// The default name for the task data file.
const TASKS_FILE_NAME: &str = "tasks.toml";

/// Determines the path where the tasks file should be stored.
///
/// For simplicity, it currently places the file in the current working directory.
/// In a real application, you might use a configuration directory (e.g., `dirs-next` crate).
fn get_tasks_file_path() -> PathBuf {
    PathBuf::from(TASKS_FILE_NAME)
}

/// Loads tasks from the tasks file.
///
/// # Returns
///
/// A `Result` containing a `Vec<Task>` by unpacking from `TaskList` on success, or an `AppError` on failure.
pub fn load_tasks() -> Result<Vec<Task>> {
    let path = get_tasks_file_path();
    debug!("Attempting to load tasks from: {}", path.display());

    if !path.exists() {
        info!(
            "Tasks file not found at {}. Returning empty list.",
            path.display()
        );
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(&path)?;
    debug!("Successfully read contents from {}.", path.display());

    // Deserialize into the wrapper struct
    let task_list: TaskList =
        toml::from_str(&contents).map_err(|e| AppError::TomlDeserialize(e))?;
    info!(
        "Successfully loaded {} tasks from {}.",
        task_list.tasks.len(),
        path.display()
    );
    Ok(task_list.tasks)
}

/// Saves the given tasks to the tasks file.
///
/// Overwrites the existing file if it exists.
///
/// # Arguments
///
/// * `tasks` - A slice of `Task` structs to be saved.
///
/// # Returns
///
/// A `Result` by wrapping in `TaskList` indicating success or an `AppError` on failure.
pub fn save_tasks(tasks: &[Task]) -> Result<()> {
    let path = get_tasks_file_path();
    debug!(
        "Attempting to save {} tasks to: {}",
        tasks.len(),
        path.display()
    );

    // Wrap the tasks slice into a TaskList struct for serialization
    let task_list = TaskList {
        tasks: tasks.to_vec(),
    };

    let contents = toml::to_string(&task_list).map_err(|e| AppError::TomlSerialize(e))?;

    fs::write(&path, contents)?;
    info!("Successfully saved tasks to {}.", path.display());

    Ok(())
}
