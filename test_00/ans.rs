// 方針: 与えられた幅と高さから面積・周囲長・正方形か否かを一度の関数呼び出しでまとめて算出する。
// 思考フロー: (1) 正方形判定に必要な比較と面積/周囲長に必要な積・和を整理する (2) u32のまま算術演算してよいか確認する (3) 3つの値を呼び出し側へ返す最小コストの手段としてタプルを選ぶ。
// 使用技術まとめ: u32の算術演算と比較、計算途中値を束ねるタプル戻り値。
// 長方形の面積・周囲長・正方形判定をu32の算術演算と比較演算で求め、タプルで返却する実装。
// 参照: https://doc.rust-jp.rs/book-ja/ch03-02-data-types.html
pub fn rectangle_report(width: u32, height: u32) -> (u32, u32, bool) {
    let area = width * height;
    let perimeter = 2 * (width + height);
    let is_square = width == height;
    return (area, perimeter, is_square);
}
