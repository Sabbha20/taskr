mod menu;
mod models;
use dialoguer::{Input, Select, theme::ColorfulTheme};

use crate::menu::MenuAction;
use crate::models::task::Priority;
use crate::models::task_list::TaskList;

// helper function to parse a string into a u32 id in both complete and remove commands
fn parse_id(arg: &str) -> Result<u32, String> {
    arg.trim()
        .parse::<u32>()
        .map_err(|_| format!("Invalid id: {arg}"))
}

fn handle_add(tasks: &mut TaskList, arg: &str, priority: Priority) -> Result<String, String> {
    let id = tasks.add(arg.trim(), priority)?;
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

fn prompt(msg: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .interact_text()
        .unwrap()
}

fn select_priority() -> Priority {
    let priority_options: Vec<&str> = Priority::ALL.iter().map(|p| p.label()).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a priority")
        .items(&priority_options)
        .default(0)
        .interact()
        .unwrap();
    Priority::ALL[selection]
}

fn main() {
    let mut tasks = TaskList::new();
    let add_task_err = "Failed to add task";
    tasks.add("Learn Rust", Priority::High).expect(add_task_err);
    tasks
        .add("Implement Rust", Priority::High)
        .expect(add_task_err);
    tasks.add("Test Rust", Priority::Low).expect(add_task_err);
    tasks
        .add("Buy some milk", Priority::Medium)
        .expect(add_task_err);

    let opt_items: Vec<&str> = MenuAction::ALL.iter().map(|a| a.label()).collect();

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .items(&opt_items)
            .default(0)
            .interact()
            .unwrap();
        let action = MenuAction::ALL[selection];

        let result = match action {
            MenuAction::Add => {
                let title = prompt("Task title");
                let priority = select_priority();
                handle_add(&mut tasks, &title, priority)
            }
            MenuAction::Complete => {
                let id = prompt("Task id");
                handle_complete(&mut tasks, &id)
            }
            MenuAction::Remove => {
                let id = prompt("Task id");
                handle_remove(&mut tasks, &id)
            }
            MenuAction::List => {
                tasks.list();
                continue;
            }
            MenuAction::Quit => {
                println!("Exiting...");
                break;
            }
        };
        match result {
            Ok(msg) => println!("{msg}"),
            Err(e) => println!("Error: {e}"),
        }
    }
}
