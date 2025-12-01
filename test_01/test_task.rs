#[path = "task.rs"]
mod task;

use task::build_greeting;

fn repeats_three_times_case() {
    let expected = "[1] Hello, Taro!\n[2] Hello, Taro!\n[3] Hello, Taro!";
    assert_eq!(build_greeting("Taro", 3), expected);
}

fn works_for_single_line_case() {
    assert_eq!(build_greeting("Rust", 1), "[1] Hello, Rust!");
}

fn returns_empty_string_for_zero_times_case() {
    assert!(build_greeting("Anyone", 0).is_empty());
}

fn supports_unicode_name_case() {
    let expected = "[1] Hello, あいう!\n[2] Hello, あいう!";
    assert_eq!(build_greeting("あいう", 2), expected);
}

#[test]
fn repeats_three_times() {
    repeats_three_times_case();
}

#[test]
fn works_for_single_line() {
    works_for_single_line_case();
}

#[test]
fn returns_empty_string_for_zero_times() {
    returns_empty_string_for_zero_times_case();
}

#[test]
fn supports_unicode_name() {
    supports_unicode_name_case();
}

#[cfg(not(test))]
fn main() {
    run_console_tests(&[
        ("repeats_three_times", repeats_three_times_case as fn()),
        ("works_for_single_line", works_for_single_line_case as fn()),
        (
            "returns_empty_string_for_zero_times",
            returns_empty_string_for_zero_times_case as fn(),
        ),
        ("supports_unicode_name", supports_unicode_name_case as fn()),
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
