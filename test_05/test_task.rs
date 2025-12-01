#[path = "task.rs"]
mod task;

use task::{describe_task, TaskState};

fn formats_todo_case() {
    let text = describe_task(TaskState::Todo("Write docs".into()));
    assert_eq!(text, "TODO: Write docs");
}

fn formats_running_case() {
    let text = describe_task(TaskState::Running {
        name: "build".into(),
        remaining: 42,
    });
    assert_eq!(text, "Running build (42s left)");
}

fn formats_blocked_case() {
    let text = describe_task(TaskState::Blocked("waiting for IO".into()));
    assert_eq!(text, "Blocked: waiting for IO");
}

fn formats_done_case() {
    assert_eq!(describe_task(TaskState::Done), "Done");
}

#[test]
fn formats_todo() {
    formats_todo_case();
}

#[test]
fn formats_running() {
    formats_running_case();
}

#[test]
fn formats_blocked() {
    formats_blocked_case();
}

#[test]
fn formats_done() {
    formats_done_case();
}

#[cfg(not(test))]
fn main() {
    run_console_tests(&[
        ("formats_todo", formats_todo_case as fn()),
        ("formats_running", formats_running_case as fn()),
        ("formats_blocked", formats_blocked_case as fn()),
        ("formats_done", formats_done_case as fn()),
    ]);
}

#[cfg(not(test))]
fn run_console_tests(tests: &[(&str, fn())]) {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    let mut failed = Vec::new();
    for (name, case) in tests {
        match catch_unwind(AssertUnwindSafe(|| case())) {
            Ok(_) => println!("PASS: {}", name),
            Err(err) => {
                println!("FAIL: {}", name);
                print_error(&err);
                failed.push(*name);
            }
        }
    }

    if failed.is_empty() {
        println!("All tests passed");
    } else {
        println!("{} test(s) failed:", failed.len());
        for name in failed {
            println!("  - {}", name);
        }
        std::process::exit(1);
    }
}

#[cfg(not(test))]
fn print_error(err: &Box<dyn std::any::Any + Send>) {
    if let Some(msg) = err.downcast_ref::<&str>() {
        println!("    {}", msg);
    } else if let Some(msg) = err.downcast_ref::<String>() {
        println!("    {}", msg);
    } else {
        println!("    <non-string panic>");
    }
}
