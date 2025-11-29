#[path = "task.rs"]
mod task;

use task::apply_pipeline;

fn filters_and_transforms_numbers_case() {
    let data = vec![1, 2, 3, 4];
    let result = apply_pipeline(data, |n| if n % 2 == 0 { Some(n * 2) } else { None });
    assert_eq!(result, vec![4, 8]);
}

fn works_with_strings_case() {
    let data = vec!["hi".to_string(), "rust".into(), "world".into()];
    let result = apply_pipeline(data, |s| if s.len() > 3 { Some(s.to_uppercase()) } else { None });
    assert_eq!(result, vec!["RUST".to_string(), "WORLD".to_string()]);
}

fn closure_can_capture_state_case() {
    let mut counter = 0;
    let data = vec![10, 20, 30, 40];
    let result = apply_pipeline(data, |n| {
        counter += 1;
        if counter % 2 == 0 {
            Some(n / 10)
        } else {
            None
        }
    });
    assert_eq!(result, vec![2, 4]);
}

fn empty_input_returns_empty_output_case() {
    let data: Vec<i32> = Vec::new();
    let result: Vec<i32> = apply_pipeline(data, |n| Some(n));
    assert!(result.is_empty());
}

#[test]
fn filters_and_transforms_numbers() {
    filters_and_transforms_numbers_case();
}

#[test]
fn works_with_strings() {
    works_with_strings_case();
}

#[test]
fn closure_can_capture_state() {
    closure_can_capture_state_case();
}

#[test]
fn empty_input_returns_empty_output() {
    empty_input_returns_empty_output_case();
}

#[cfg(not(test))]
fn main() {
    run_console_tests(&[
        (
            "filters_and_transforms_numbers",
            filters_and_transforms_numbers_case as fn(),
        ),
        ("works_with_strings", works_with_strings_case as fn()),
        ("closure_can_capture_state", closure_can_capture_state_case as fn()),
        (
            "empty_input_returns_empty_output",
            empty_input_returns_empty_output_case as fn(),
        ),
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
