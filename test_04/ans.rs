// 方針: 1行のCSV風文字列を安全にi32 Vecへ変換し、空入力や不正トークンを専用エラー型で分類する。
// 思考フロー: (1) 先にtrimで全体空白を削除し、空ならEmptyエラー (2) split(',')で各トークンを取り出し、毎回trim+空チェック (3) parse::<i32>()が失敗したらInvalidエラーにトークン文字列を残して返す。
// 使用技術まとめ: 独自enumエラー定義、String::trim/str::splitによる整形、Resultと?・map_errでのエラー伝播、Vecへのpush。
// 参照: https://doc.rust-jp.rs/book-ja/ch06-01-defining-an-enum.html, https://doc.rust-jp.rs/book-ja/ch09-02-recoverable-errors-with-result.html, https://doc.rust-lang.org/std/primitive.str.html#method.split
/// 入力行全体のエラー分類。Display実装は不要だがDebug比較をderiveしてテストしやすくする。
#[derive(Debug, PartialEq, Eq)]
pub enum ParseScoreError {
    /// 空行など有効なトークンが無かった場合。
    Empty,
    /// 個別トークンが空or数値変換失敗だった場合に、問題の文字列を保持する。
    Invalid(String),
}

// &strをsplitしResultと?演算子で早期returnするパーサ実装で、map_errにより独自エラーへ変換。
// 参照: https://doc.rust-jp.rs/book-ja/ch09-02-recoverable-errors-with-result.html
/// 1行文字列をカンマ区切りで解析し、i32配列として返す。
pub fn parse_score_line(line: &str) -> Result<Vec<i32>, ParseScoreError> {
    // line.trim()で前後空白を除去し、分析対象を正規化する。
    let trimmed = line.trim();
    // 完全な空行だった場合はEmptyエラーを即返却し、以降の処理を避ける。
    if trimmed.is_empty() {
        // エラーのバリアントEmptyを返して早期終了。ResultのErr側を選ぶ。
        return Err(ParseScoreError::Empty);
    }
    // 成功値を蓄積するVecを用意。Vec::new()は空の可変配列。
    let mut values = Vec::new();
    // split(',')で各カンマ区切りトークン(&str)を得て、forで順に処理。
    for raw in trimmed.split(',') {
        // 各トークンもtrimして前後空白を取り除く。
        let token = raw.trim();
        // トークン自体が空ならInvalidエラーを返す。to_stringでStringを所有させる。
        if token.is_empty() {
            // token.to_string()で&strから所有Stringを作り、Invalidに格納する。
            return Err(ParseScoreError::Invalid(token.to_string()));
        }
        // parse::<i32>()を呼び、map_errで標準エラーをParseScoreError::Invalidへ変換しつつ?で伝播。
        let value = token
            .parse::<i32>()
            .map_err(|_| ParseScoreError::Invalid(token.to_string()))?;
        // 変換できた値を結果Vecへpushする。
        values.push(value);
    }
    // すべて成功したらOkでVecを包んで返却。
    Ok(values)
}
