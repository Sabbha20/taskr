use std::fmt;

pub struct Task {
    id: u32,
    title: String,
    done: bool,
}

impl Task {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn new(id: u32, title: &str) -> Self {
        Self {
            id,
            title: String::from(title),
            done: false,
        }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mark = if self.done { "[x]" } else { "[ ]" };
        write!(f, "{mark} Task {}: {}", self.id, self.title)
    }
}
