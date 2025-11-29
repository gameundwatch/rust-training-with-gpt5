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

#[test]
fn counts_simple_sentence() {
    let counts = word_tally("Rust is fast and Rust is friendly");
    assert_eq!(counts, map_of(&[("rust", 2), ("is", 2), ("fast", 1), ("and", 1), ("friendly", 1)]));
}

#[test]
fn strips_punctuation() {
    let counts = word_tally("Hello, world! Hello???");
    assert_eq!(counts.get("hello"), Some(&2));
    assert_eq!(counts.get("world"), Some(&1));
}

#[test]
fn ignores_empty_segments() {
    let counts = word_tally(" ..RUST.. ");
    assert_eq!(counts.len(), 1);
}

#[test]
fn handles_mixed_case_and_numbers() {
    let counts = word_tally("CPU cpu Cpu123 123cpu");
    assert_eq!(counts.get("cpu"), Some(&2));
    assert_eq!(counts.get("cpu123"), Some(&1));
    assert_eq!(counts.get("123cpu"), Some(&1));
}
