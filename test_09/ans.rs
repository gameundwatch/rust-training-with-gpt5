// 方針: Vec<T>を受け取り、各要素にクロージャを適用してOption<U>を返させ、Someだけを抽出するパイプライン関数を作る。
// 思考フロー: (1) 関数にジェネリック型T/UとFnMut境界を設定して汎用化 (2) Vec::into_iterで元Vecを消費しつつクロージャへ所有権を渡す (3) if letでSomeを判定し、output Vecにpushする。
// 使用技術まとめ: ジェネリクス、FnMutトレイト境界、Option/if let構文、Vec::new/Vec::push。
// 参照: https://doc.rust-jp.rs/book-ja/ch10-01-syntax.html, https://doc.rust-lang.org/std/option/enum.Option.html, https://doc.rust-jp.rs/book-ja/ch18-03-pattern-syntax.html#if-let
/// 入力Vecと変換クロージャを受け取り、Someだけを集めたVecにして返す。
pub fn apply_pipeline<T, U, F>(input: Vec<T>, mut f: F) -> Vec<U>
where
    // クロージャfはTを受け取ってOption<U>を返す可変クロージャ(FnMut)であると指定する。
    F: FnMut(T) -> Option<U>,
{
    // 結果を蓄積する空のVec<U>を用意。mutにしpushできるようにする。
    let mut output = Vec::new();
    // into_iterでinputから要素の所有権を取り出し、itemへ1つずつ渡す。
    for item in input.into_iter() {
        // if let Some(value) = ... でSomeの場合のみvalueを取り出し、body内を実行する。
        if let Some(value) = f(item) {
            // Someだったものだけ結果Vecへpushする。valueの所有権はVecへ移る。
            output.push(value);
        }
    }
    // 収集し終えたVecを返す。
    output
}
