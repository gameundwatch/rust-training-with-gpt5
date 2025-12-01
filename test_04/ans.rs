// 方針: 入力行をカンマ区切りでパースし、空要素や数値化失敗を独自エラー型で表現する堅牢な関数を提供する。
// 思考フロー: (1) 空行かどうか最初に判定する (2) splitとtrimで各トークンを整形し、その場でバリデーションする (3) parseで得た結果をVecに蓄積し、途中失敗はResultで即返す。
// 使用技術まとめ: カスタムenumエラー、split/trimによる文字列整形、Result/`?`/map_errでのエラー伝播。
// 入力の妥当性分類に列挙型を使い、各変種に詳細情報を保持するエラー表現。
// 参照: https://doc.rust-jp.rs/book-ja/ch06-01-defining-an-enum.html
#[derive(Debug, PartialEq, Eq)]
pub enum ParseScoreError {
    Empty,
    Invalid(String),
}

// &strをsplitしResultと?演算子で早期returnするパーサ実装で、map_errにより独自エラーへ変換。
// 参照: https://doc.rust-jp.rs/book-ja/ch09-02-recoverable-errors-with-result.html
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
