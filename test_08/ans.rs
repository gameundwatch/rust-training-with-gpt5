// ファイル入出力で`fs::read_to_string`を使うためにstd::fsをインポート。
// 参照: https://doc.rust-lang.org/std/fs/fn.read_to_string.html
use std::fs;
// io::Resultとio::Errorを使ってエラー情報を伝える。
use std::io;
// 呼び出し側が扱うPath引数を参照で受け取るためにstd::path::Pathを使用。
use std::path::Path;

// 方針: ファイルを一括で読み出し、空行を除いた各行をi64へparseして合計、失敗時は行番号付きio::Errorで知らせる。
// 思考フロー: (1) Pathからfs::read_to_stringで内容を得る (2) lines().enumerate()で1-origin行番号を把握しtrim/空チェック (3) parse::<i64>()失敗時はio::Error::newでInvalidData種別へ包んで?で伝播。
// 使用技術まとめ: Path→String読込、str::lines + Iterator::enumerate、trimとis_empty、parse::<i64>()、io::Result + ?エラー伝播。
// 参照: https://doc.rust-jp.rs/book-ja/ch12-03-improving-error-handling-and-modularity.html, https://doc.rust-jp.rs/book-ja/ch09-02-recoverable-errors-with-result.html, https://doc.rust-lang.org/std/primitive.str.html#method.trim
/// ファイル内の数値行を合計し、無効な行を検知したらio::Errorでエラーにする。
pub fn sum_file_numbers(path: &Path) -> io::Result<i64> {
    // fs::read_to_stringでファイル全体をUTF-8として読み取る。?でio::ResultのErrをそのまま返す。
    let contents = fs::read_to_string(path)?;
    // 合計値を64bit符号付きで初期化。0からスタート。
    let mut total = 0i64;
    // lines()で行イテレータを得て、enumerate()で0-based行番号を取り、idx, lineタプルに展開する。
    for (idx, line) in contents.lines().enumerate() {
        // line.trim()で前後空白や改行を除去し、純粋な数字部分を得る。
        let trimmed = line.trim();
        // 空行は意味がないのでcontinueでスキップ。
        if trimmed.is_empty() {
            continue;
        }
        // parse::<i64>()で文字列を数値化。map_errでエラー時にio::Errorへ変換し、行番号をメッセージに含める。
        let value = trimmed.parse::<i64>().map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid number on line {}: {}", idx + 1, trimmed),
            )
        })?;
        // 取得した値を合計に加算。
        total += value;
    }
    // 正常終了の場合はOk(total)を返し、合計値を呼び出し元に届ける。
    Ok(total)
}
