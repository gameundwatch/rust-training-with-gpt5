// 方針: 幅と高さの2入力から面積・周囲長・正方形判定を一括で算出し、再利用しやすい関数にまとめる。
// 思考フロー: (1) それぞれの指標(面積/周囲長/正方形判定)に必要な算術演算を整理 (2) u32範囲で溢れない前提を確認しそのまま演算 (3) 3値をまとめる最小構造としてタプルを採用しreturnする。
// 使用技術まとめ: 整数リテラル・積和・比較といった基本算術、タプルによる多値返却、return式。
// 参照: https://doc.rust-jp.rs/book-ja/ch03-02-data-types.html, https://doc.rust-lang.org/std/primitive.u32.html
/// 幅と高さから面積・周囲長・正方形判定の3値をタプルで返すレポート関数。
pub fn rectangle_report(width: u32, height: u32) -> (u32, u32, bool) {
    // widthはu32の所有値として受け取り、不変なのでそのまま使う。
    let area = width * height;
    // width * heightで長方形の面積を求める。u32の乗算は32bit符号なし整数同士の積を返す。
    let perimeter = 2 * (width + height);
    // 周囲長は (幅+高さ)*2 なので先に和を求めてから定数2を掛ける。u32同士の加算と乗算を組み合わせている。
    let is_square = width == height;
    // 等辺比較`==`で正方形か判定する。u32はPartialEqを実装しているため直接比較できる。
    return (area, perimeter, is_square);
    // 3値を(面積, 周囲長, bool)のタプルで返す。タプルは複数値をまとめられるRust組み込み型。
}
