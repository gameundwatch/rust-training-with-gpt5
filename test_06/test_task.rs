#[path = "task.rs"]
mod task;

use task::normalize_and_sort;

fn trims_and_lowercases_case() {
    let data = vec!["  Apple".into(), "banana  ".into(), "APPLE".into()];
    let result = normalize_and_sort(data);
    assert_eq!(result, vec!["apple".to_string(), "banana".to_string()]);
}

fn filters_empty_entries_case() {
    let data = vec![" ".into(), "Rust".into(), "".into()];
    let result = normalize_and_sort(data);
    assert_eq!(result, vec!["rust".to_string()]);
}

fn handles_already_sorted_list_case() {
    let data = vec!["a".into(), "b".into(), "c".into()];
    assert_eq!(
        normalize_and_sort(data),
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
}

fn sorts_descending_input_case() {
    let data = vec!["Zulu".into(), "Mike".into(), "alpha".into(), " mike".into()];
    let result = normalize_and_sort(data);
    assert_eq!(result, vec!["alpha".to_string(), "mike".to_string(), "zulu".to_string()]);
}

#[test]
fn trims_and_lowercases() {
    trims_and_lowercases_case();
}

#[test]
fn filters_empty_entries() {
    filters_empty_entries_case();
}

#[test]
fn handles_already_sorted_list() {
    handles_already_sorted_list_case();
}

#[test]
fn sorts_descending_input() {
    sorts_descending_input_case();
}

#[cfg(not(test))]
fn main() {
    run_console_tests(&[
        ("trims_and_lowercases", trims_and_lowercases_case as fn()),
        ("filters_empty_entries", filters_empty_entries_case as fn()),
        ("handles_already_sorted_list", handles_already_sorted_list_case as fn()),
        ("sorts_descending_input", sorts_descending_input_case as fn()),
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
