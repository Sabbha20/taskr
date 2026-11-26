#[derive(Clone, Copy)]
pub enum MenuAction {
    Add,
    List,
    ListByPriority,
    ReverseList,
    ListPending,
    Complete,
    Remove,
    Quit,
    
}
impl MenuAction {
    pub const ALL: [MenuAction; 8] = [
        MenuAction::Add,
        MenuAction::List,
        MenuAction::ListByPriority,
        MenuAction::ReverseList,
        MenuAction::ListPending,
        MenuAction::Complete,
        MenuAction::Remove,
        MenuAction::Quit,
    ];

    pub fn label(&self) -> &str {
        match self {
            MenuAction::Add => "Add",
            MenuAction::List => "List",
            MenuAction::ListByPriority => "List by Priority",
            MenuAction::ReverseList => "Reverse List",
            MenuAction::ListPending => "List Pending",
            MenuAction::Complete => "Complete",
            MenuAction::Remove => "Remove",
            MenuAction::Quit => "Quit",
        }
    }
}
