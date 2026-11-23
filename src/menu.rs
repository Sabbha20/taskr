#[derive(Clone, Copy, PartialEq, Eq)] 
pub enum MenuAction { Add, List, Complete, Remove, Quit }
impl MenuAction {
    pub const ALL: [MenuAction; 5] = [
        MenuAction::Add, MenuAction::List, MenuAction::Complete,
        MenuAction::Remove, MenuAction::Quit,
    ];

    pub fn label(&self) -> &str {
        match self {
            MenuAction::Add => "Add",
            MenuAction::List => "List",
            MenuAction::Complete => "Complete",
            MenuAction::Remove => "Remove",
            MenuAction::Quit => "Quit",
        }
    }
}