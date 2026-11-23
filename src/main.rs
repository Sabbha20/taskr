mod models;
use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::models::task_list::TaskList;

#[derive(Clone, Copy, PartialEq, Eq)] 
enum MenuAction { Add, List, Complete, Remove, Quit }
impl MenuAction {
    const ALL: [MenuAction; 5] = [
        MenuAction::Add, MenuAction::List, MenuAction::Complete,
        MenuAction::Remove, MenuAction::Quit,
    ];

    fn label(&self) -> &str {
        match self {
            MenuAction::Add => "Add",
            MenuAction::List => "List",
            MenuAction::Complete => "Complete",
            MenuAction::Remove => "Remove",
            MenuAction::Quit => "Quit",
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

fn prompt(msg: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .interact_text()
        .unwrap()
}

fn main() {
    let mut tasks = TaskList::new();
    let add_task_err = "Failed to add task";
    tasks.add("Learn Rust").expect(add_task_err);
    tasks.add("Implement Rust").expect(add_task_err);
    tasks.add("Test Rust").expect(add_task_err);
    tasks.add("Buy some milk").expect(add_task_err);

    let opt_items: Vec<&str> = MenuAction::ALL.iter().map(|a| a.label()).collect();

    loop {
        let selection =Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .items(&opt_items)
        .default(0)
        .interact()
        .unwrap();
        let action = MenuAction::ALL[selection];
        
        let result = match action {
            MenuAction::Add => {
                let title: String = prompt("Task title");
                handle_add(&mut tasks, &title)
            },
            MenuAction::Complete => {
                let id: String = prompt("Task id");
                handle_complete(&mut tasks, &id)
            },
            MenuAction::Remove => {
                let id: String = prompt("Task id");
                handle_remove(&mut tasks, &id)
            },
            MenuAction::List => { tasks.list(); continue; }
            MenuAction::Quit => { println!("Exiting..."); break; }
        };
        match result {
            Ok(msg) => println!("{msg}"),
            Err(e) => println!("Error: {e}"),
        }
    }


}
