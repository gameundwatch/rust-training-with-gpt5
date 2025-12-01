// 方針: 呼び出し元からもらった名前と繰り返し回数をもとに、インデックス付き挨拶を改行区切りでつなげた文字列を生成する。
// 思考フロー: (1) 毎回Stringを再生成せずに1本の可変Stringへpushする (2) 0-basedインデックスをforループで得て、1-based表記に変換して印字 (3) 最初の行だけ改行を挿入しない条件を設ける。
// 使用技術まとめ: String::newとpush/push_str、範囲forループ、format!マクロでの整形、if条件での改行制御。
// 参照: https://doc.rust-jp.rs/book-ja/ch08-02-strings.html, https://doc.rust-lang.org/std/macro.format.html
/// 指定回数分の挨拶行を作り、`[n] Hello, name!`形式を改行区切りで返す。
pub fn build_greeting(name: &str, times: usize) -> String {
    // newで空のStringバッファを確保。mutableにして後続で文字を追加できるようにする。
    let mut result = String::new();
    // 0..timesの半開区間をforで回し、インデックスidxを取得。timesが0ならループは実行されない。
    for idx in 0..times {
        // 2行目以降は直前に改行を挟む。idx > 0がその判定。charリテラル'\n'をpushしている。
        if idx > 0 {
            result.push('\n');
        }
        // format!で`[番号] Hello, 名前!`の1行分を組み立てる。idx+1で1始まり表記に変換し、push_strで末尾に追加。
        result.push_str(&format!("[{}] Hello, {}!", idx + 1, name));
    }
    // 最終的な連結結果の所有権を呼び出し元へ返す。
    result
}
