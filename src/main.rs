mod models;

use crate::models::task_list::TaskList;

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
            "\n--------------------------\n Enter a command\n--------------------------\n > `add` <title> adds a task.\n > `list` or `ls` shows the tasks.\n > `complete` or `mark` <id> marks a task as complete.\n > `remove` <id> removes a task.\n > `quit` or `exit` or `q` exits."
        );
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let input = user_input.trim();
        let (cmd, arg) = input.split_once(' ').unwrap_or((input, ""));
        if cmd == "list" || cmd == "ls" {
            tasks.list();
            continue;
        }
        if cmd == "quit" || cmd == "exit" || cmd == "q" {
            println!("Exiting...");
            break;
        }
        let result = match cmd {
            "add" => handle_add(&mut tasks, arg),
            "complete" | "mark" => handle_complete(&mut tasks, arg),
            "remove" => handle_remove(&mut tasks, arg),
            _ => Err(format!("Unknown command: {}", input)),
        };
        match result {
            Ok(msg) => println!("{msg}"),
            Err(e) => println!("Error: {e}"),
        }
    }
}
