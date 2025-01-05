use crate::task::Task;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn read_file(file_path: &str) -> Result<Vec<Task>, Box<dyn Error>> {
    let json_path = Path::new(file_path);

    let file = match File::open(json_path) {
        Ok(file) => file,
        Err(_) => {
            let mut new_file = File::create(json_path)?;
            new_file.write_all(b"[]")?;
            return Ok(Vec::new());
        }
    };

    let tasks: Vec<Task> = serde_json::from_reader(file)?;

    Ok(tasks)
}

pub fn write_file(file_path: &str, tasks: &Vec<Task>) -> Result<(), Box<dyn Error>> {
    let json_path = Path::new(file_path);

    let file = match File::create(json_path) {
        Ok(file) => file,
        Err(_) => {
            return Err("Error creating file".into());
        }
    };

    serde_json::to_writer(file, tasks)?;

    Ok(())
}
