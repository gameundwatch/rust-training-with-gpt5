use std::collections::HashMap;

// 問題: 文中に出現する語を小文字で集計する関数を完成させてください。
// - `text` を空白や記号で分割し、英数字以外は区切りとみなします (`split` + クロージャで表現できます)。
// - 各語を `to_lowercase()` で揃え、`HashMap<String, usize>` に件数をカウントします。
// - 空文字列は無視し、結果のマップを返してください。
// - HashMap の `entry` API を使うと集計しやすくなります。標準ライブラリの所有権移動に注意しましょう。
pub fn word_tally(text: &str) -> HashMap<String, usize> {
    todo!("ここに処理を書いてください")
}
