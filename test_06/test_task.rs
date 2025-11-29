#[path = "task.rs"]
mod task;

use task::normalize_and_sort;

#[test]
fn trims_and_lowercases() {
    let data = vec!["  Apple".into(), "banana  ".into(), "APPLE".into()];
    let result = normalize_and_sort(data);
    assert_eq!(result, vec!["apple".to_string(), "banana".to_string()]);
}

#[test]
fn filters_empty_entries() {
    let data = vec![" ".into(), "Rust".into(), "".into()];
    let result = normalize_and_sort(data);
    assert_eq!(result, vec!["rust".to_string()]);
}

#[test]
fn handles_already_sorted_list() {
    let data = vec!["a".into(), "b".into(), "c".into()];
    assert_eq!(
        normalize_and_sort(data),
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
}

#[test]
fn sorts_descending_input() {
    let data = vec!["Zulu".into(), "Mike".into(), "alpha".into(), " mike".into()];
    let result = normalize_and_sort(data);
    assert_eq!(result, vec!["alpha".to_string(), "mike".to_string(), "zulu".to_string()]);
}
