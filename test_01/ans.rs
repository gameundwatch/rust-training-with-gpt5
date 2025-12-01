// 方針: 繰り返し数を指定し、1件ずつ整形した挨拶を改行区切りで連結する文字列生成関数を作る。
// 思考フロー: (1) 可変なStringにpushしていくのがシンプルか検討 (2) インデックスを持つループで番号付けを行う (3) 最初の行だけ改行を避ける条件を入れる。
// 使用技術まとめ: 可変String、forループと範囲、format!マクロとpush/push_strによる文字列連結。
// 可変なStringバッファを構築し、forループとformat!マクロで繰り返し連結する処理。
// 参照: https://doc.rust-jp.rs/book-ja/ch08-02-strings.html
pub fn build_greeting(name: &str, times: usize) -> String {
    let mut result = String::new();
    for idx in 0..times {
        if idx > 0 {
            result.push('\n');
        }
        result.push_str(&format!("[{}] Hello, {}!", idx + 1, name));
    }
    result
}
