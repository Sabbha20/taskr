mod models;
use dialoguer::{theme::ColorfulTheme, Input, Select};

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

    let opt_items = vec!["Add", "List", "Complete", "Remove", "Quit"];

    loop {
        let selection =Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .items(&opt_items)
        .default(0)
        .interact()
        .unwrap();
        
        if selection == 1 {
            tasks.list();
            continue;
        }
        if selection == 4 {
            println!("Exiting...");
            break;
        }
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let input = user_input.trim();
        let result = match selection {
            0 => handle_add(&mut tasks, input),
            2 => handle_complete(&mut tasks, input),
            3 => handle_remove(&mut tasks, input),
            _ => Err(format!("Unknown command: {}", input)),
        };
        match result {
            Ok(msg) => println!("{msg}"),
            Err(e) => println!("Error: {e}"),
        }
    }


}
