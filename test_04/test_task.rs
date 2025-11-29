#[path = "task.rs"]
mod task;

use task::{parse_score_line, ParseScoreError};

#[test]
fn parses_valid_line() {
    let scores = parse_score_line("12, 34,56").unwrap();
    assert_eq!(scores, vec![12, 34, 56]);
}

#[test]
fn handles_negative_numbers_and_spaces() {
    let scores = parse_score_line(" -1 , 0 , 5 ").unwrap();
    assert_eq!(scores, vec![-1, 0, 5]);
}

#[test]
fn errors_on_empty_input() {
    assert_eq!(parse_score_line("   "), Err(ParseScoreError::Empty));
}

#[test]
fn errors_on_invalid_token() {
    let err = parse_score_line("10, xx, 3").unwrap_err();
    assert_eq!(err, ParseScoreError::Invalid("xx".into()));
}
