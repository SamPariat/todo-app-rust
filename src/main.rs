mod file;
mod task;

use crate::file::{read_file, write_file};
use crate::task::{NewTask, Task, TaskGettersSetters};
use clap::{Parser, Subcommand};
use std::alloc::System;
use std::error::Error;
use std::time::SystemTime;

const FILE_PATH: &str = "./tasks.json";

#[derive(Subcommand, Debug)]
enum TaskAction {
    GetAll,
    GetById {
        id: i64,
    },
    Add {
        title: String,
        description: String,
        is_completed: Option<bool>,
    },
    UpdateStatus {
        id: i64,
        is_completed: Option<bool>,
    },
    Delete {
        id: i64,
    },
}

#[derive(Parser)]
#[command(name = "todo-app")]
#[command(about = "A simple todo application")]
struct Cli {
    #[command(subcommand)]
    action: TaskAction,
}

fn get_all_tasks() -> Result<(), Box<dyn std::error::Error>> {
    let all_tasks = read_file(FILE_PATH)?;

    println!("All tasks {:?}", all_tasks);

    Ok(())
}

fn get_task_by_id(id: i64) -> Result<Option<Task>, Box<dyn std::error::Error>> {
    let all_tasks = read_file(FILE_PATH)?;

    for task in all_tasks {
        if task.get_id() == id {
            return Ok(Some(task));
        }
    }

    Ok(None)
}

fn add_task(task: &Task) -> Result<(), Box<dyn std::error::Error>> {
    let mut all_tasks = read_file(FILE_PATH)?;

    all_tasks.push(task.clone());

    write_file(FILE_PATH, &all_tasks)
}

fn update_task_status(id: i64, completed: bool) -> Result<(), Box<dyn std::error::Error>> {
    match get_task_by_id(id) {
        Ok(None) => {
            println!("No task found with id {:?}", id);
            return Ok(());
        }
        Err(err) => {
            println!("Error: {:?}", err);
            return Err("Error updating task".into());
        }
        _ => {}
    }

    let mut all_tasks = read_file(FILE_PATH)?;

    for task in all_tasks.iter_mut() {
        if task.get_id() == id {
            task.set_is_completed(completed);
        }
    }

    write_file(FILE_PATH, &all_tasks)?;

    Ok(())
}

fn delete_task(id: i64) -> Result<(), Box<dyn std::error::Error>> {
    let mut all_tasks = read_file(FILE_PATH)?;

    all_tasks.retain(|task| task.get_id() != id);

    write_file(FILE_PATH, &all_tasks)?;

    Ok(())
}

fn main() {
    let args = Cli::parse();

    match args.action {
        TaskAction::GetAll => match get_all_tasks() {
            Ok(_) => {}
            Err(err) => {
                println!("Error: {}", err);
            }
        },
        TaskAction::GetById { id } => match get_task_by_id(id) {
            Ok(Some(task)) => println!("Task details {:?}", task),
            Ok(None) => println!("No task found with id {:?}", id),
            Err(err) => println!("Error: {}", err),
        },
        TaskAction::Add {
            title,
            description,
            is_completed,
        } => {
            let mut completed = false;

            match is_completed {
                Some(true) => completed = true,
                _ => {}
            }

            let now = SystemTime::now();
            let duration = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
            let epoch_seconds = duration.as_secs() as i64;

            let new_task = Task::new(epoch_seconds, title, description, completed);

            add_task(&new_task).unwrap_or_else(|err| println!("Error: {}", err));
        }
        TaskAction::UpdateStatus { id, is_completed } => {
            let mut completed = false;

            match is_completed {
                Some(true) => completed = true,
                _ => {}
            }

            update_task_status(id, completed).unwrap_or_else(|err| println!("Error: {}", err));
        }
        TaskAction::Delete { id } => {
            delete_task(id).unwrap_or_else(|err| println!("Error: {}", err));
        }
    }
}
