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

#[test]
fn sums_positive_numbers() {
    let file = TempFile::new("1\n2\n3\n").unwrap();
    let sum = sum_file_numbers(&file.path).unwrap();
    assert_eq!(sum, 6);
}

#[test]
fn ignores_empty_lines() {
    let file = TempFile::new("10\n\n -5 \n").unwrap();
    let sum = sum_file_numbers(&file.path).unwrap();
    assert_eq!(sum, 5);
}

#[test]
fn reports_invalid_line() {
    let file = TempFile::new("1\nnope\n2").unwrap();
    let err = sum_file_numbers(&file.path).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::InvalidData);
}

#[test]
fn works_with_large_values() {
    let file = TempFile::new("1000000\n-1\n1").unwrap();
    assert_eq!(sum_file_numbers(&file.path).unwrap(), 1_000_000);
}
