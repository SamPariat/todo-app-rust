use serde::{Deserialize, Serialize};

pub trait NewTask {
    fn new(id: i64, title: String, description: String, is_completed: bool) -> Self;
}

pub trait TaskGettersSetters {
    fn get_id(&self) -> i64;
    fn set_id(&mut self, id: i64);
    fn get_title(&self) -> String;
    fn set_title(&mut self, title: String);
    fn get_description(&self) -> String;
    fn set_description(&mut self, description: String);
    fn get_is_completed(&self) -> bool;
    fn set_is_completed(&mut self, is_completed: bool);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    id: i64,
    title: String,
    description: String,
    is_completed: bool,
}

impl NewTask for Task {
    fn new(id: i64, title: String, description: String, is_completed: bool) -> Self {
        Self {
            id,
            title,
            description,
            is_completed,
        }
    }
}

impl TaskGettersSetters for Task {
    fn get_id(&self) -> i64 {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = id;
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn set_title(&mut self, title: String) {
        self.title = title
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn set_description(&mut self, description: String) {
        self.description = description
    }

    fn get_is_completed(&self) -> bool {
        self.is_completed
    }

    fn set_is_completed(&mut self, is_completed: bool) {
        self.is_completed = is_completed
    }
}
