// 方針: スライス内の偶数のみ平均を計算し、対象が無ければNoneを返す安全な集計器を作る。
// 思考フロー: (1) 偶数判定が必要なのでループ処理で各要素を見る (2) 合計と件数を別々に集計し0件時の扱いを決める (3) 平均はf64にキャストして算出しOptionで包む。
// 使用技術まとめ: forループでのスライス走査、整数の型変換、Optionによる値有無の表現。
// スライスをforで走査し偶数のみ合計・件数を集計、値が存在しなければOption::Noneを返す平均計算。
// 参照: https://doc.rust-jp.rs/book-ja/ch06-01-defining-an-enum.html
pub fn average_even(numbers: &[i32]) -> Option<f64> {
    let mut sum = 0i64;
    let mut count = 0u32;
    for &n in numbers {
        if n % 2 == 0 {
            sum += n as i64;
            count += 1;
        }
    }
    if count == 0 {
        None
    } else {
        Some(sum as f64 / count as f64)
    }
}
