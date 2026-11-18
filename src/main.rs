use std::fmt;

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

struct TaskList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl Task {
    fn new(id: u32, title: &str) -> Self {
        Self {
            id,
            title: String::from(title),
            done: false,
        }
    }

    fn mark_done(&mut self) {
        self.done = true;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mark = if self.done { "[x]" } else { "[ ]" };
        write!(f, "{mark} Task {}: {}", self.id, self.title)
    }
}

impl TaskList {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, title: &str) {
        let task = Task::new(self.next_id, title);
        self.tasks.push(task);
        self.next_id += 1;
    }

    fn list(&self) {
        for task in &self.tasks {
            println!("{}", task);
        }
    }

    fn complete(&mut self, id: u32) -> Result<(), String> {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.mark_done();
                Ok(())
            }
            None => Err(format!("Task {id} not found"))
        }
    }
}

fn main() {
    let mut tasks = TaskList::new();
    tasks.add("Learn Rust");
    tasks.add("Implement Rust");
    tasks.add("Test Rust");

    tasks.complete(2).unwrap_or_else(|e| println!("Error: {e}"));

    tasks.list();
    tasks.complete(99).unwrap_or_else(|e| println!("Error: {e}"));
    match tasks.complete(100){
        Ok(_) => println!("Task 100 completed"),
        Err(e) => println!("Error: {e}"),
    }
    tasks.list();
}
