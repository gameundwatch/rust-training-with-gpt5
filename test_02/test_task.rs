#[path = "task.rs"]
mod task;

use task::average_even;

#[test]
fn computes_average_for_mixed_values() {
    let nums = [3, 4, 10, -2, 5];
    let avg = average_even(&nums).unwrap();
    assert!((avg - 4.0).abs() < f64::EPSILON);
}

#[test]
fn returns_none_when_no_even() {
    let nums = [1, 3, 5];
    assert_eq!(average_even(&nums), None);
}

#[test]
fn handles_negative_even_numbers() {
    let nums = [-4, -2, 1, -6];
    let avg = average_even(&nums).unwrap();
    assert!((avg + 4.0).abs() < f64::EPSILON);
}

#[test]
fn single_even_value() {
    assert_eq!(average_even(&[7, 2, 9]), Some(2.0));
}
