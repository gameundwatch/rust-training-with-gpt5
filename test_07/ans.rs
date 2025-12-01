// 方針: 数列スライスとウィンドウ幅を受け取り、幅ごとの連続部分和をVecに収集する。
// 思考フロー: (1) windowサイズ0やスライス長超過時は結果が無いので空Vecを返す (2) slice::windows(window)で固定長の部分スライスを得る (3) 各windowに対しiter().sum()で合計しcollectする。
// 使用技術まとめ: 範囲チェックによる早期return、スライスのwindowsメソッド、Iterator::map + sum、collect()。
// 参照: https://doc.rust-lang.org/std/primitive.slice.html#method.windows, https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
/// スライディングウィンドウ方式で連続部分和を求める。
pub fn sliding_window_sum(values: &[i64], window: usize) -> Vec<i64> {
    // windowが0またはvalues.len()を超えると有効なウィンドウが作れないため、空Vecを即返す。
    if window == 0 || window > values.len() {
        return Vec::new();
    }
    values
        // windows(window)で長さwindowの部分スライス&[i64]を順番に生成する。
        .windows(window)
        // mapで各チャンクをi64に変換。chunk.iter().sum()で部分スライスの総和を計算する。
        .map(|chunk| chunk.iter().sum())
        // イテレータをVec<i64>に収集する。
        .collect()
}
