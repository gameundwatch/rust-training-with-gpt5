#[path = "task.rs"]
mod task;

use task::apply_pipeline;

#[test]
fn filters_and_transforms_numbers() {
    let data = vec![1, 2, 3, 4];
    let result = apply_pipeline(data, |n| if n % 2 == 0 { Some(n * 2) } else { None });
    assert_eq!(result, vec![4, 8]);
}

#[test]
fn works_with_strings() {
    let data = vec!["hi".to_string(), "rust".into(), "world".into()];
    let result = apply_pipeline(data, |s| if s.len() > 3 { Some(s.to_uppercase()) } else { None });
    assert_eq!(result, vec!["RUST".to_string(), "WORLD".to_string()]);
}

#[test]
fn closure_can_capture_state() {
    let mut counter = 0;
    let data = vec![10, 20, 30, 40];
    let result = apply_pipeline(data, |n| {
        counter += 1;
        if counter % 2 == 0 {
            Some(n / 10)
        } else {
            None
        }
    });
    assert_eq!(result, vec![2, 4]);
}

#[test]
fn empty_input_returns_empty_output() {
    let data: Vec<i32> = Vec::new();
    let result: Vec<i32> = apply_pipeline(data, |n| Some(n));
    assert!(result.is_empty());
}
