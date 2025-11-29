use std::fs;
use std::io;
use std::path::Path;

pub fn sum_file_numbers(path: &Path) -> io::Result<i64> {
    let contents = fs::read_to_string(path)?;
    let mut total = 0i64;
    for (idx, line) in contents.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let value = trimmed.parse::<i64>().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid number on line {}: {}", idx + 1, trimmed),
            )
        })?;
        total += value;
    }
    Ok(total)
}
