use crate::models::task::{Priority, Task};
use std::cmp::Reverse;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TaskList {
    tasks: Vec<Task>,
    next_id: u32,
}

fn print_tasks(title: &str, tasks: &[&Task]) {
    println!("+{}+", "-".repeat(90));
    println!("|{:^90}|", title);
    println!("+{}+", "-".repeat(90));
    for task in tasks {
        println!("| {:<89}|", task.to_string());
    }
    println!("+{}+", "-".repeat(90));
}

impl TaskList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, title: &str, priority: Priority) -> Result<u32, String> {
        if title.trim().is_empty() {
            return Err("Task title cannot be empty".to_string());
        }
        let id = self.next_id;
        let task = Task::new(id, title, priority);
        self.tasks.push(task);
        self.next_id += 1;
        Ok(id)
    }

    pub fn remove(&mut self, id: u32) -> Result<Task, String> {
        match self.tasks.iter().position(|t| t.id() == id) {
            Some(index) => Ok(self.tasks.remove(index)),
            None => Err(format!("Task {id} not found")),
        }
    }

    pub fn list(&self) {
        let all_items: Vec<&Task> = self.tasks.iter().collect();
        print_tasks("Task List", &all_items);
    }

    pub fn list_by_priority(&self) {
        let mut sorted: Vec<&Task> = self.tasks.iter().collect();
        sorted.sort_by_key(|t| t.priority());
        print_tasks("Task List by Low Priority", &sorted);
    }

    pub fn reverse_list(&self) {
        let mut reversed: Vec<&Task> = self.tasks.iter().collect();
        reversed.sort_by_key(|t| Reverse(t.priority()));
        print_tasks("Task List in High Priority", &reversed);
    }

    pub fn list_pending(&self) {
        let pending_items: Vec<&Task> = self.tasks.iter().filter(|t| !t.is_done()).collect();
        print_tasks("Pending Tasks", &pending_items);
    }

    pub fn complete(&mut self, id: u32) -> Result<(), String> {
        match self.tasks.iter_mut().find(|t| t.id() == id) {
            Some(task) => {
                task.mark_done();
                Ok(())
            }
            None => Err(format!("Task {id} not found")),
        }
    }

    pub fn save(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self)
            .map_err(|e| format!("Failed to serialize tasks: {}", e))?;
        std::fs::write(path, json).map_err(|e| format!("Failed to write to file: {}", e))?;
        Ok(())
    }
}

impl Default for TaskList {
    fn default() -> Self {
        Self::new()
    }
}
