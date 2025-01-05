#[derive(Debug, Clone)]
struct Task {
    id: i32,
    title: String,
    description: String,
    is_completed: bool,
}

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn get_all_tasks(all_tasks: &Vec<Task>) {
    println!("Getting all tasks...");

    for task in all_tasks {
        println!("Task: {:?}", *task);
    }
}

fn get_task_by_id(all_tasks: &Vec<Task>, id: i32) -> Option<&Task> {
    println!("Getting task by id {:?}...", id);

    for task in all_tasks {
        if task.id == id {
            println!("This is the task {:?}", task);
            return Some(task);
        }
    }

    println!("No task found for id {:?}", id);
    None
}

fn add_task(all_tasks: &mut Vec<Task>, task: &Task) {
    println!("Adding a new task: {:?}", task);
    all_tasks.push(task.clone());
}

fn update_task<'a>(all_tasks: &'a mut Vec<Task>, updated_task: &Task) -> Option<&'a Task> {
    println!("Updating task with id {:?}...", updated_task.id);

    for task in all_tasks {
        if task.id == updated_task.id {
            println!("This is the task {:?}", task);
            *task = updated_task.clone();
            return Some(task);
        }
    }

    println!("No task found for id {:?}", updated_task.id);
    None
}

fn delete_task(all_tasks: &mut Vec<Task>, id: i32) {
    println!("Deleting task with id {:?}...", id);

    let original_length = all_tasks.len();

    all_tasks.retain(|task| {
        if task.id == id {
            println!("This is the task {:?}", task);
            false
        } else {
            true
        }
    });

    if original_length == all_tasks.len() {
        println!("No task found for id {:?}", id);
    }
}

fn main() {
    let mut all_tasks: Vec<Task> = Vec::new();

    get_all_tasks(&all_tasks);

    add_task(
        &mut all_tasks,
        &Task {
            id: 1,
            title: "First".to_string(),
            description: "This is the first task".to_string(),
            is_completed: false,
        },
    );

    get_all_tasks(&all_tasks);

    get_task_by_id(&all_tasks, 0);

    get_task_by_id(&all_tasks, 1);

    update_task(
        &mut all_tasks,
        &Task {
            id: 1,
            title: "Updated title".to_string(),
            description: "Updated description".to_string(),
            is_completed: true,
        },
    );

    get_all_tasks(&all_tasks);

    delete_task(&mut all_tasks, 1);

    get_all_tasks(&all_tasks);
}
