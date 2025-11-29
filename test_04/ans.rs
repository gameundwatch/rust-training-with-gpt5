#[derive(Debug, PartialEq, Eq)]
pub enum ParseScoreError {
    Empty,
    Invalid(String),
}

pub fn parse_score_line(line: &str) -> Result<Vec<i32>, ParseScoreError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err(ParseScoreError::Empty);
    }
    let mut values = Vec::new();
    for raw in trimmed.split(',') {
        let token = raw.trim();
        if token.is_empty() {
            return Err(ParseScoreError::Invalid(token.to_string()));
        }
        let value = token
            .parse::<i32>()
            .map_err(|_| ParseScoreError::Invalid(token.to_string()))?;
        values.push(value);
    }
    Ok(values)
}
