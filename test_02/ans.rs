// 方針: 整数スライスから偶数だけを対象に平均値を求め、対象がなければNoneで知らせる安全な統計関数を提供する。
// 思考フロー: (1) 合計と件数を別変数で持ち、偶数判定を満たした値だけ集計 (2) 整数のままでは平均が小数点以下を失うのでf64へ型変換 (3) 件数0の場合は割り算を避けOption::Noneを返す。
// 使用技術まとめ: forループによるスライス走査、`%`演算子で偶数判定、`as`キャストで型変換、Optionでの有無表現。
// 参照: https://doc.rust-jp.rs/book-ja/ch03-02-data-types.html, https://doc.rust-jp.rs/book-ja/ch06-01-defining-an-enum.html
/// 偶数だけを対象に平均を計算し、存在しないときはNoneを返す。
pub fn average_even(numbers: &[i32]) -> Option<f64> {
    // sumはi64で持ち、i32同士を足したときのオーバーフロー余裕を確保する。
    let mut sum = 0i64;
    // countは整数個数なのでu32で保持する。0からスタートし、偶数を見つけるたびに増やす。
    let mut count = 0u32;
    // for &n in numbers でスライス要素をコピー(n: i32)し、各要素を順番に処理する。
    for &n in numbers {
        // n % 2 == 0 で偶数か判定。余りが0なら偶数。
        if n % 2 == 0 {
            sum += n as i64;
            // nをi64にキャストし合計へ足す。asで数値型を変換できる。
            count += 1;
            // 件数を1つ進める。+=は加算代入演算子。
        }
    }
    // 件数0なら平均不可なのでNoneで早期returnする。
    if count == 0 {
        // Option::Noneを返して「偶数が存在しなかった」という情報を伝える。
        None
    } else {
        // sum / count をf64で計算するため、それぞれf64へキャストしてから割り算し、Someで包む。
        Some(sum as f64 / count as f64)
    }
}
