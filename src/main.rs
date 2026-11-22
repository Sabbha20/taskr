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

    fn add(&mut self, title: &str) -> Result<u32, String> {
        if title.trim().is_empty() {
            return Err("Task title cannot be empty".to_string());
        }
        let id = self.next_id;
        let task = Task::new(id, title);
        self.tasks.push(task);
        self.next_id += 1;
        Ok(id)
    }

    fn remove(&mut self, id: u32) -> Result<Task, String> {
        match self.tasks.iter().position(|t| t.id == id) {
            Some(index) => Ok(self.tasks.remove(index)),
            None => Err(format!("Task {id} not found")),
        }
    }

    fn list(&self) {
        println!("+----------------------------------------+");
        println!("|             Task List:                 |");
        println!("+----------------------------------------+");
        for task in &self.tasks {
            println!("{}", task);
        }
        println!("==========================================");
    }

    fn complete(&mut self, id: u32) -> Result<(), String> {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.mark_done();
                Ok(())
            }
            None => Err(format!("Task {id} not found")),
        }
    }
}

// helper function to parse a string into a u32 id in both complete and remove commands
fn parse_id(arg: &str) -> Result<u32, String> {
    arg.trim()
        .parse::<u32>()
        .map_err(|_| format!("Invalid id: {arg}"))
}

fn handle_add(tasks: &mut TaskList, arg: &str) -> Result<String, String> {
    let id = tasks.add(arg.trim())?;
    Ok(format!("Task added with id: {id}"))
}

fn handle_complete(tasks: &mut TaskList, arg: &str) -> Result<String, String> {
    let id = parse_id(arg)?;
    tasks.complete(id)?;
    Ok(format!("Task {id} completed"))
}

fn handle_remove(tasks: &mut TaskList, arg: &str) -> Result<String, String> {
    let id = parse_id(arg)?;
    let task = tasks.remove(id)?;
    Ok(format!("Removed: {task}"))
}

fn main() {
    let mut tasks = TaskList::new();
    let add_task_err = "Failed to add task";
    tasks.add("Learn Rust").expect(add_task_err);
    tasks.add("Implement Rust").expect(add_task_err);
    tasks.add("Test Rust").expect(add_task_err);
    tasks.add("Buy some milk").expect(add_task_err);

    loop {
        println!(
            "\n--------------------------\n Enter a command\n--------------------------\n > `add` <title> adds a task.\n > `list` shows the tasks.\n > `complete` <id> marks a task as complete.\n > `remove` <id> removes a task.\n > `quit` exits."
        );
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let input = user_input.trim();
        let (cmd, arg) = input.split_once(' ').unwrap_or((input, ""));
        if cmd == "list" {
            tasks.list();
            continue;
        }
        if cmd == "quit" || cmd == "exit" {
            println!("Exiting...");
            break;
        }
        let result = match cmd {
            "add" => handle_add(&mut tasks, arg),
            "complete" => handle_complete(&mut tasks, arg),
            "remove" => handle_remove(&mut tasks, arg),
            _ => Err(format!("Unknown command: {}", input)),
        };
        match result {
            Ok(msg) => println!("{msg}"),
            Err(e) => println!("Error: {e}"),
        }
    }
}
