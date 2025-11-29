#[derive(Debug, PartialEq, Eq)]
pub enum TaskState {
    Todo(String),
    Running { name: String, remaining: u32 },
    Blocked(String),
    Done,
}

pub fn describe_task(state: TaskState) -> String {
    match state {
        TaskState::Todo(text) => format!("TODO: {}", text),
        TaskState::Running { name, remaining } => {
            format!("Running {} ({}s left)", name, remaining)
        }
        TaskState::Blocked(reason) => format!("Blocked: {}", reason),
        TaskState::Done => "Done".to_string(),
    }
}
