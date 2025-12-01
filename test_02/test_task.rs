#[path = "task.rs"]
mod task;

use task::average_even;

fn computes_average_for_mixed_values_case() {
    let nums = [3, 4, 10, -2, 5];
    let avg = average_even(&nums).unwrap();
    assert!((avg - 4.0).abs() < f64::EPSILON);
}

fn returns_none_when_no_even_case() {
    let nums = [1, 3, 5];
    assert_eq!(average_even(&nums), None);
}

fn handles_negative_even_numbers_case() {
    let nums = [-4, -2, 1, -6];
    let avg = average_even(&nums).unwrap();
    assert!((avg + 4.0).abs() < f64::EPSILON);
}

fn single_even_value_case() {
    assert_eq!(average_even(&[7, 2, 9]), Some(2.0));
}

#[test]
fn computes_average_for_mixed_values() {
    computes_average_for_mixed_values_case();
}

#[test]
fn returns_none_when_no_even() {
    returns_none_when_no_even_case();
}

#[test]
fn handles_negative_even_numbers() {
    handles_negative_even_numbers_case();
}

#[test]
fn single_even_value() {
    single_even_value_case();
}

#[cfg(not(test))]
fn main() {
    run_console_tests(&[
        ("computes_average_for_mixed_values", computes_average_for_mixed_values_case as fn()),
        ("returns_none_when_no_even", returns_none_when_no_even_case as fn()),
        ("handles_negative_even_numbers", handles_negative_even_numbers_case as fn()),
        ("single_even_value", single_even_value_case as fn()),
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
