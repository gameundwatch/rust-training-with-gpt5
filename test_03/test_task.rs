#[path = "task.rs"]
mod task;

use std::collections::HashMap;
use task::word_tally;

fn map_of(pairs: &[(&str, usize)]) -> HashMap<String, usize> {
    pairs
        .iter()
        .map(|(word, count)| (word.to_string(), *count))
        .collect()
}

fn counts_simple_sentence_case() {
    let counts = word_tally("Rust is fast and Rust is friendly");
    assert_eq!(counts, map_of(&[("rust", 2), ("is", 2), ("fast", 1), ("and", 1), ("friendly", 1)]));
}

fn strips_punctuation_case() {
    let counts = word_tally("Hello, world! Hello???");
    assert_eq!(counts.get("hello"), Some(&2));
    assert_eq!(counts.get("world"), Some(&1));
}

fn ignores_empty_segments_case() {
    let counts = word_tally(" ..RUST.. ");
    assert_eq!(counts.len(), 1);
}

fn handles_mixed_case_and_numbers_case() {
    let counts = word_tally("CPU cpu Cpu123 123cpu");
    assert_eq!(counts.get("cpu"), Some(&2));
    assert_eq!(counts.get("cpu123"), Some(&1));
    assert_eq!(counts.get("123cpu"), Some(&1));
}

#[test]
fn counts_simple_sentence() {
    counts_simple_sentence_case();
}

#[test]
fn strips_punctuation() {
    strips_punctuation_case();
}

#[test]
fn ignores_empty_segments() {
    ignores_empty_segments_case();
}

#[test]
fn handles_mixed_case_and_numbers() {
    handles_mixed_case_and_numbers_case();
}

#[cfg(not(test))]
fn main() {
    run_console_tests(&[
        ("counts_simple_sentence", counts_simple_sentence_case as fn()),
        ("strips_punctuation", strips_punctuation_case as fn()),
        ("ignores_empty_segments", ignores_empty_segments_case as fn()),
        ("handles_mixed_case_and_numbers", handles_mixed_case_and_numbers_case as fn()),
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
