#[path = "task.rs"]
mod task;

use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use task::sum_file_numbers;

struct TempFile {
    path: PathBuf,
}

impl TempFile {
    fn new(contents: &str) -> io::Result<Self> {
        let mut path = std::env::temp_dir();
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!("rust_training_sum_file_{}_{}.txt", std::process::id(), unique));
        fs::write(&path, contents)?;
        Ok(TempFile { path })
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn sums_positive_numbers_case() {
    let file = TempFile::new("1\n2\n3\n").unwrap();
    let sum = sum_file_numbers(&file.path).unwrap();
    assert_eq!(sum, 6);
}

fn ignores_empty_lines_case() {
    let file = TempFile::new("10\n\n -5 \n").unwrap();
    let sum = sum_file_numbers(&file.path).unwrap();
    assert_eq!(sum, 5);
}

fn reports_invalid_line_case() {
    let file = TempFile::new("1\nnope\n2").unwrap();
    let err = sum_file_numbers(&file.path).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::InvalidData);
}

fn works_with_large_values_case() {
    let file = TempFile::new("1000000\n-1\n1").unwrap();
    assert_eq!(sum_file_numbers(&file.path).unwrap(), 1_000_000);
}

#[test]
fn sums_positive_numbers() {
    sums_positive_numbers_case();
}

#[test]
fn ignores_empty_lines() {
    ignores_empty_lines_case();
}

#[test]
fn reports_invalid_line() {
    reports_invalid_line_case();
}

#[test]
fn works_with_large_values() {
    works_with_large_values_case();
}

#[cfg(not(test))]
fn main() {
    run_console_tests(&[
        ("sums_positive_numbers", sums_positive_numbers_case as fn()),
        ("ignores_empty_lines", ignores_empty_lines_case as fn()),
        ("reports_invalid_line", reports_invalid_line_case as fn()),
        ("works_with_large_values", works_with_large_values_case as fn()),
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
