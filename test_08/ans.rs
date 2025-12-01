use std::fs;
use std::io;
use std::path::Path;

// 方針: ファイルを読み込み、各行を数値に変換して合計し、行番号付きのエラーで失敗を報告する。
// 思考フロー: (1) `fs::read_to_string`でまとめて読み込んでよいか確認 (2) linesを列挙して空行をスキップしtrim→parseを行う (3) parse失敗時にio::Errorへ変換し、?で呼び出し元へ伝播する。
// 使用技術まとめ: ファイル入出力、文字列→整数のparse、enumerateで行番号管理、io::Resultと?によるエラー伝播。
// fs::read_to_stringでファイル全体を読み込み、lines→trim→parseでi64へ変換し、Resultと?でエラー伝播を行う集計処理。
// 参照: https://doc.rust-jp.rs/book-ja/ch12-03-improving-error-handling-and-modularity.html, https://doc.rust-jp.rs/book-ja/ch09-02-recoverable-errors-with-result.html
pub fn sum_file_numbers(path: &Path) -> io::Result<i64> {
    let contents = fs::read_to_string(path)?;
    let mut total = 0i64;
    for (idx, line) in contents.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let value = trimmed.parse::<i64>().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid number on line {}: {}", idx + 1, trimmed),
            )
        })?;
        total += value;
    }
    Ok(total)
}
