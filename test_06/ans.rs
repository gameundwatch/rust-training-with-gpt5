// 方針: 所有している文字列Vecを受け取り、個々の要素をtrim+小文字化したうえで空文字を除外し、ソート・重複排除したリストを返す。
// 思考フロー: (1) into_iterで要素所有権を取りながらmapで正規化 (2) filterで空文字を落としcollectでVec化 (3) sort()→dedup()で順序と重複削除を確定させる。
// 使用技術まとめ: Iteratorチェーン、String::trim/to_lowercase、Vec::sort/Vec::dedup。
// 参照: https://doc.rust-jp.rs/book-ja/ch13-02-iterators.html, https://doc.rust-lang.org/std/string/struct.String.html#method.trim, https://doc.rust-lang.org/std/vec/struct.Vec.html#method.dedup
/// 文字列リストを正規化・整列・重複排除した新しいVecにして返す。
pub fn normalize_and_sort(items: Vec<String>) -> Vec<String> {
    // cleanedは後でソートする必要があるためmutableにする。collectの型推論を助けるためVec<String>を注釈。
    let mut cleaned: Vec<String> = items
        // into_iter()で元Vecの所有権を消費し、イテレータに各Stringを流す。
        .into_iter()
        // mapクロージャ内でtrim→to_lowercaseを行い、新しい正規化済みStringを生成する。
        .map(|s| s.trim().to_lowercase())
        // trimで前後空白を落とし、その結果を小文字化して返す。
        .filter(|s| !s.is_empty())
        // filterで空でないものだけ残す。クロージャは&String参照を受け取り真偽を返す。
        .collect();
    // collect()でイテレータをVec<String>にまとめる。型注釈によりターゲット型を指定済み。
    cleaned.sort();
    // sort()でアルファベット順に並び替える。StringはOrdを実装しているため比較可能。
    cleaned.dedup();
    // dedup()で隣接する重複要素を削除。事前にsortしておくことで全重複を隣接させている。
    cleaned
    // 整形済みVecの所有権を返す。
}
