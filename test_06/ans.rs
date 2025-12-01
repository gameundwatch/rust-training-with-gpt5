// 方針: 文字列リストを正規化(前後空白削除・小文字化)し、空要素を捨てた上でソート＆重複排除した結果を返す。
// 思考フロー: (1) into_iterで所有権を取りつつmap/filterで変換処理を並べる (2) Vecにcollectしてから並べ替える (3) sort→dedupの順で重複除去する必要がある。
// 使用技術まとめ: イテレータチェーン(map/filter/collect)、Stringのtrim/to_lowercase、Vecのsortとdedup。
// into_iter→map→filter→collectのイテレータ連鎖で正規化し、sort/dedupで順序付けと重複排除を行う。
// 参照: https://doc.rust-jp.rs/book-ja/ch13-02-iterators.html
pub fn normalize_and_sort(items: Vec<String>) -> Vec<String> {
    let mut cleaned: Vec<String> = items
        .into_iter()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();
    cleaned.sort();
    cleaned.dedup();
    cleaned
}
