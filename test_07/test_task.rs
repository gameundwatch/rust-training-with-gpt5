#[path = "task.rs"]
mod task;

use task::sliding_window_sum;

fn computes_basic_windows_case() {
    let result = sliding_window_sum(&[1, 2, 3, 4], 2);
    assert_eq!(result, vec![3, 5, 7]);
}

fn handles_full_window_case() {
    let result = sliding_window_sum(&[5, 5, 5], 3);
    assert_eq!(result, vec![15]);
}

fn returns_empty_when_window_too_large_case() {
    assert!(sliding_window_sum(&[1, 2], 4).is_empty());
}

fn returns_empty_when_window_zero_case() {
    assert!(sliding_window_sum(&[1, 2, 3], 0).is_empty());
}

fn works_with_negative_numbers_case() {
    let result = sliding_window_sum(&[-2, 4, -1, 3], 3);
    assert_eq!(result, vec![1, 6]);
}

#[test]
fn computes_basic_windows() {
    computes_basic_windows_case();
}

#[test]
fn handles_full_window() {
    handles_full_window_case();
}

#[test]
fn returns_empty_when_window_too_large() {
    returns_empty_when_window_too_large_case();
}

#[test]
fn returns_empty_when_window_zero() {
    returns_empty_when_window_zero_case();
}

#[test]
fn works_with_negative_numbers() {
    works_with_negative_numbers_case();
}

#[cfg(not(test))]
fn main() {
    run_console_tests(&[
        ("computes_basic_windows", computes_basic_windows as fn()),
        ("handles_full_window", handles_full_window as fn()),
        (
            "returns_empty_when_window_too_large",
            returns_empty_when_window_too_large as fn(),
        ),
        (
            "returns_empty_when_window_zero",
            returns_empty_when_window_zero as fn(),
        ),
        ("works_with_negative_numbers", works_with_negative_numbers as fn()),
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
