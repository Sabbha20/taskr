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

fn main() {
    let mut tasks = TaskList::new();
    tasks.add("Learn Rust");
    tasks.add("Implement Rust");
    tasks.add("Test Rust");
    tasks.add("Buy some milk");

    // tasks.complete(2).unwrap_or_else(|e| println!("Error: {e}"));

    // tasks.list();
    // tasks
    //     .complete(99)
    //     .unwrap_or_else(|e| println!("Error: {e}"));
    // match tasks.complete(100) {
    //     Ok(_) => println!("Task 100 completed"),
    //     Err(e) => println!("Error: {e}"),
    // }

    // match tasks.remove(1) {
    //     Ok(mut task) => {
    //         task.mark_done();
    //         println!("Removed: {task}");
    //     },
    //     Err(e) => println!("Error: {e}"),
    // }
    // tasks.list();

    let mut user_input;
    loop {
        println!("\n--------------------------\n Enter a command\n--------------------------\n > `add` <title> adds a task.\n > `list` shows the tasks.\n > `complete` <id> marks a task as complete.\n > `remove` <id> removes a task.\n > `quit` exits.");
        user_input = String::new();
        std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let cmd = user_input.trim().split_once(' ');
        match cmd {
            Some(("add", title)) => tasks.add(title.trim()),
            Some(("list", _)) => tasks.list(),
            Some(("complete", id_str)) => {
                match id_str.trim().parse::<u32>() {
                    Ok(id) => match tasks.complete(id) {
                        Ok(_) => println!("Task {id} completed"),
                        Err(e) => println!("Error: {e}"),
                    },
                    Err(e) => println!("Invalid id: {id_str}: {e}"),
                }
            },
            Some(("remove", id_str)) => {
                match id_str.trim().parse::<u32>() {
                    Ok(id) => match tasks.remove(id) {
                        Ok(task) => println!("Removed: {task}"),
                        Err(e) => println!("Error: {e}"),
                    },
                    Err(e) => println!("Invalid id: {id_str}: {e}"),
                }
            },
            Some(("quit", _)) => {
                println!("Exiting...");
                break;
            },
            _ => println!("Unknown command: {}", user_input.trim()),
        }

    }

}
