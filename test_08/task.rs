use std::fs;
use std::io;
use std::path::Path;

// 問題: ファイルから 1 行ずつ整数を読み取り、合計を計算してください。
// - `path` で指定されたファイルを `fs::read_to_string` で読み込みます。
// - 各行を `trim()` し、空行は無視します。残った行は i64 として `parse` し、合計に加算します。
// - 変換できない行があれば `io::Error::new(io::ErrorKind::InvalidData, メッセージ)` で失敗させてください。
// - すべて成功したら合計を `Ok(total)` で返します。ファイル IO と `Result` の組み合わせ練習です。
pub fn sum_file_numbers(path: &Path) -> io::Result<i64> {
    todo!("ここに処理を書いてください")
}
