#[path = "task.rs"]
mod task;

use task::{describe_task, TaskState};

#[test]
fn formats_todo() {
    let text = describe_task(TaskState::Todo("Write docs".into()));
    assert_eq!(text, "TODO: Write docs");
}

#[test]
fn formats_running() {
    let text = describe_task(TaskState::Running {
        name: "build".into(),
        remaining: 42,
    });
    assert_eq!(text, "Running build (42s left)");
}

#[test]
fn formats_blocked() {
    let text = describe_task(TaskState::Blocked("waiting for IO".into()));
    assert_eq!(text, "Blocked: waiting for IO");
}

#[test]
fn formats_done() {
    assert_eq!(describe_task(TaskState::Done), "Done");
}
