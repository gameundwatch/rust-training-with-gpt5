#[path = "task.rs"]
mod task;

use task::build_greeting;

#[test]
fn repeats_three_times() {
    let expected = "[1] Hello, Taro!\n[2] Hello, Taro!\n[3] Hello, Taro!";
    assert_eq!(build_greeting("Taro", 3), expected);
}

#[test]
fn works_for_single_line() {
    assert_eq!(build_greeting("Rust", 1), "[1] Hello, Rust!");
}

#[test]
fn returns_empty_string_for_zero_times() {
    assert!(build_greeting("Anyone", 0).is_empty());
}

#[test]
fn supports_unicode_name() {
    let expected = "[1] Hello, あいう!\n[2] Hello, あいう!";
    assert_eq!(build_greeting("あいう", 2), expected);
}
