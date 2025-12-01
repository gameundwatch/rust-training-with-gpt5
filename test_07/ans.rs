// 方針: 固定サイズのスライディングウィンドウで部分和を計算し、結果のベクタを返す。
// 思考フロー: (1) 無効なwindowサイズ(0や長さ超過)の扱いを決める (2) slice::windowsを用いて連続部分列の参照を得る (3) 各チャンクをiter::sumで合計しcollectする。
// 使用技術まとめ: スライスAPIwindows、iter::sum、collectでVec化、早期returnで境界条件を処理。
// スライスのwindowsメソッドで固定長部分列を生成し、各チャンクのiter::sumで加算した畳み込み的処理。
// 参照: https://doc.rust-lang.org/std/primitive.slice.html#method.windows
pub fn sliding_window_sum(values: &[i64], window: usize) -> Vec<i64> {
    if window == 0 || window > values.len() {
        return Vec::new();
    }
    values
        .windows(window)
        .map(|chunk| chunk.iter().sum())
        .collect()
}
