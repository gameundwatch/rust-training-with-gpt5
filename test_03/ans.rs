// HashMapコレクションを使い、単語→出現回数を保存するためにstd::collectionsから導入する。
// 参照: https://doc.rust-lang.org/std/collections/struct.HashMap.html
use std::collections::HashMap;

// 方針: 任意のテキストから単語を切り出し、小文字へ統一したキーでHashMapに出現回数を蓄積する。
// 思考フロー: (1) 単語区切りを文字判定クロージャで定義しsplitへ渡す (2) 空文字はskipして余計なカウントを避ける (3) HashMap::entry APIでキーの存在に応じてカウンタを更新する。
// 使用技術まとめ: 文字列イテレータ(split)、to_lowercaseでの正規化、HashMap::new/entry/or_insertによる頻度集計。
// 参照: https://doc.rust-jp.rs/book-ja/ch08-03-hash-maps.html, https://doc.rust-lang.org/std/primitive.str.html#method.split
/// テキストを単語単位に分解し、小文字化した単語の頻度表をHashMapで返す。
pub fn word_tally(text: &str) -> HashMap<String, usize> {
    // HashMap::newで空の頻度表を作成。キーがString、値がusize(個数)。
    let mut counts = HashMap::new();
    // splitにクロージャを渡し、英数字以外の文字で区切る。結果は単語スライスのイテレータ。
    for raw in text.split(|c: char| !c.is_alphanumeric()) {
        // 空文字はノイズなのでcontinueでスキップ。
        if raw.is_empty() {
            continue;
        }
        // to_lowercaseでUnicode対応の小文字変換を行い、Stringを得る。
        let word = raw.to_lowercase();
        // entry(word)で該当キーのエントリを取得し、or_insert(0)でなければ0を差し込み、*で可変参照を解決して+=1する。
        *counts.entry(word).or_insert(0) += 1;
    }
    // 出来上がったHashMapを返却する。
    counts
}
