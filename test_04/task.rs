// 問題: 1 行のカンマ区切り文字列を解析して数値ベクターに変換してください。
// - `line` を `,` で分割し、各トークンを `trim()` してから i32 に変換します。
// - 何も書かれていない (空文字や空白だけ) 場合は `ParseScoreError::Empty` を返してください。
// - 変換に失敗した場合は `ParseScoreError::Invalid(問題のトークン文字列)` を返します。
// - 成功した場合は `Vec<i32>` を `Ok` で返してください。`Result` と `?` 演算子の使い方を確認しましょう。
#[derive(Debug, PartialEq, Eq)]
pub enum ParseScoreError {
    Empty,
    Invalid(String),
}

pub fn parse_score_line(line: &str) -> Result<Vec<i32>, ParseScoreError> {
    // todo!("ここに処理を書いてください")
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err(ParseScoreError::Empty);
    }

    let vec = trimmed
        .split(',')
        .map(|raw| {
            let token = raw.trim();
            token
                .parse()
                .map_err(|_| ParseScoreError::Invalid(token.to_string()))
        })
        .collect::<Result<Vec<i32>, _>>();

    vec
}
