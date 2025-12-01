// 方針: 汎用的なパイプラインとして、入力Vecを受け取りクロージャでOption<U>に変換しSomeだけ収集する高階関数を提供する。
// 思考フロー: (1) クロージャのライフタイムと可変借用を考慮しFnMut境界を設定 (2) 所有権を消費しつつVec→Vecへの変換を行う (3) if letでSomeだけpushする流れを整える。
// 使用技術まとめ: ジェネリクスとトレイト境界、FnMutクロージャ、Optionを使った条件付きpush、Vec::into_iter。
// ジェネリクスT,UとFnMutクロージャを受け取り、Optionで条件付き追加を行うイテレータ風の処理。
// 参照: https://doc.rust-jp.rs/book-ja/ch10-01-syntax.html
pub fn apply_pipeline<T, U, F>(input: Vec<T>, mut f: F) -> Vec<U>
where
    F: FnMut(T) -> Option<U>,
{
    let mut output = Vec::new();
    for item in input.into_iter() {
        if let Some(value) = f(item) {
            output.push(value);
        }
    }
    output
}
