#[path = "task.rs"]
mod task;

use task::rectangle_report;

#[test]
fn reports_regular_rectangle() {
    assert_eq!(rectangle_report(3, 5), (15, 16, false));
}

#[test]
fn detects_square() {
    assert_eq!(rectangle_report(1, 1), (1, 4, true));
}

#[test]
fn handles_wide_shape() {
    assert_eq!(rectangle_report(9, 2), (18, 22, false));
}

#[test]
fn handles_zero_area() {
    assert_eq!(rectangle_report(0, 7), (0, 14, false));
}
