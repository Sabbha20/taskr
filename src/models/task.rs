use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    pub const ALL: [Priority; 3] = [Priority::Low, Priority::Medium, Priority::High];

    pub fn label(&self) -> &str {
        match self {
            Priority::High => "High",
            Priority::Low => "Low",
            Priority::Medium => "Medium",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    id: u32,
    title: String,
    done: bool,
    priority: Priority,
}

impl Task {
    pub fn id(&self) -> u32 {
        self.id
    }

    // Additional methods for Task can be added here, such as getters for done status, and priority.
    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn priority(&self) -> Priority {
        self.priority
    }

    pub fn new(id: u32, title: &str, priority: Priority) -> Self {
        Self {
            id,
            title: String::from(title),
            done: false,
            priority,
        }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mark = if self.done { "[x]" } else { "[ ]" };
        write!(
            f,
            "{mark} Task {}: {} ({})",
            self.id, self.title, self.priority
        )
    }
}
