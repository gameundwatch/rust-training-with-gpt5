// 問題: ジェネリックとクロージャの扱いに慣れましょう。
// - `apply_pipeline` は Vec<T> と `FnMut(T) -> Option<U>` を受け取り、
//   クロージャが `Some` を返した要素だけを集めて Vec<U> にします。
// - ループ中で所有権を `into_iter()` で移動し、`match` で `Some/None` を分岐させましょう。
// - 結果の Vec の順序は元の順番を維持してください。
pub fn apply_pipeline<T, U, F>(input: Vec<T>, mut f: F) -> Vec<U>
where
    F: FnMut(T) -> Option<U>,
{
    todo!("ここに処理を書いてください")
}
