#[path = "task.rs"]
mod task;

use task::{parse_score_line, ParseScoreError};

fn parses_valid_line_case() {
    let scores = parse_score_line("12, 34,56").unwrap();
    assert_eq!(scores, vec![12, 34, 56]);
}

fn handles_negative_numbers_and_spaces_case() {
    let scores = parse_score_line(" -1 , 0 , 5 ").unwrap();
    assert_eq!(scores, vec![-1, 0, 5]);
}

fn errors_on_empty_input_case() {
    assert_eq!(parse_score_line("   "), Err(ParseScoreError::Empty));
}

fn errors_on_invalid_token_case() {
    let err = parse_score_line("10, xx, 3").unwrap_err();
    assert_eq!(err, ParseScoreError::Invalid("xx".into()));
}

#[test]
fn parses_valid_line() {
    parses_valid_line_case();
}

#[test]
fn handles_negative_numbers_and_spaces() {
    handles_negative_numbers_and_spaces_case();
}

#[test]
fn errors_on_empty_input() {
    errors_on_empty_input_case();
}

#[test]
fn errors_on_invalid_token() {
    errors_on_invalid_token_case();
}

#[cfg(not(test))]
fn main() {
    run_console_tests(&[
        ("parses_valid_line", parses_valid_line_case as fn()),
        ("handles_negative_numbers_and_spaces", handles_negative_numbers_and_spaces_case as fn()),
        ("errors_on_empty_input", errors_on_empty_input_case as fn()),
        ("errors_on_invalid_token", errors_on_invalid_token_case as fn()),
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
