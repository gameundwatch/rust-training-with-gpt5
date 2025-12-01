use std::collections::HashMap;

// 方針: 文字列を単語ごとに区切り、小文字へ正規化した後にHashMapで出現回数をカウントする。
// 思考フロー: (1) 単語の区切りをどう扱うか決め、splitにクロージャを渡す (2) 空文字は除外する (3) 小文字化したキーでHashMap entry APIを使い加算する。
// 使用技術まとめ: 文字列splitのイテレータ、to_lowercaseによる正規化、HashMap::entryでの頻度集計。
// HashMapで単語頻度を管理し、splitとイテレータ処理で単語を正規化して集計するテキスト解析。
// 参照: https://doc.rust-jp.rs/book-ja/ch08-03-hash-maps.html
pub fn word_tally(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for raw in text.split(|c: char| !c.is_alphanumeric()) {
        if raw.is_empty() {
            continue;
        }
        let word = raw.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}
