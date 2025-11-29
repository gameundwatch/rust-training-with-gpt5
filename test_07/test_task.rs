#[path = "task.rs"]
mod task;

use task::sliding_window_sum;

#[test]
fn computes_basic_windows() {
    let result = sliding_window_sum(&[1, 2, 3, 4], 2);
    assert_eq!(result, vec![3, 5, 7]);
}

#[test]
fn handles_full_window() {
    let result = sliding_window_sum(&[5, 5, 5], 3);
    assert_eq!(result, vec![15]);
}

#[test]
fn returns_empty_when_window_too_large() {
    assert!(sliding_window_sum(&[1, 2], 4).is_empty());
}

#[test]
fn returns_empty_when_window_zero() {
    assert!(sliding_window_sum(&[1, 2, 3], 0).is_empty());
}

#[test]
fn works_with_negative_numbers() {
    let result = sliding_window_sum(&[-2, 4, -1, 3], 3);
    assert_eq!(result, vec![1, 6]);
}
