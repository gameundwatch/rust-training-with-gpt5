#[path = "task.rs"]
mod task;

use task::rectangle_report;

fn reports_regular_rectangle_case() {
    assert_eq!(rectangle_report(3, 5), (15, 16, false));
}

fn detects_square_case() {
    assert_eq!(rectangle_report(1, 1), (1, 4, true));
}

fn handles_wide_shape_case() {
    assert_eq!(rectangle_report(9, 2), (18, 22, false));
}

fn handles_zero_area_case() {
    assert_eq!(rectangle_report(0, 7), (0, 14, false));
}

#[test]
fn reports_regular_rectangle() {
    reports_regular_rectangle_case();
}

#[test]
fn detects_square() {
    detects_square_case();
}

#[test]
fn handles_wide_shape() {
    handles_wide_shape_case();
}

#[test]
fn handles_zero_area() {
    handles_zero_area_case();
}

#[cfg(not(test))]
fn main() {
    run_console_tests(&[
        ("reports_regular_rectangle", reports_regular_rectangle_case as fn()),
        ("detects_square", detects_square_case as fn()),
        ("handles_wide_shape", handles_wide_shape_case as fn()),
        ("handles_zero_area", handles_zero_area_case as fn()),
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
