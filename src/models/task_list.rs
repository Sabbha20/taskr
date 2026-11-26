use crate::models::task::{Priority, Task};
use std::cmp::Reverse;

pub struct TaskList {
    tasks: Vec<Task>,
    next_id: u32,
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
        println!("+----------------------------------------+");
        println!("|             Task List:                 |");
        println!("+----------------------------------------+");
        for task in &self.tasks {
            println!("{}", task);
        }
        println!("==========================================");
    }

    pub fn list_by_priority(&self) {
        let mut sorted: Vec<&Task> = self.tasks.iter().collect();
        sorted.sort_by_key(|t| t.priority());
        println!("+----------------------------------------+");
        println!("|         Task List by Low Priority:         |");
        println!("+----------------------------------------+");
        for task in sorted {
            println!("{}", task);
        }
        println!("==========================================");
    }

    pub fn reverse_list(&self) {
        let mut reversed: Vec<&Task> = self.tasks.iter().collect();
        reversed.sort_by_key(|t| Reverse(t.priority()));

        println!("+----------------------------------------+");
        println!("|         Task List in High Priority:          |");
        println!("+----------------------------------------+");
        for task in reversed {
            println!("{}", task);
        }
        println!("==========================================");
    }

    pub fn list_pending(&self) {
        println!("+----------------------------------------+");
        println!("|         Pending Task List:             |");
        println!("+----------------------------------------+");
        for task in self.tasks.iter().filter(|t| !t.done()) {
            println!("{}", task);
        }
        println!("==========================================");
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
}

impl Default for TaskList {
    fn default() -> Self {
        Self::new()
    }
}
