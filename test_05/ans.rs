// 方針: タスクの進捗を表すenumを定義し、それぞれの状態から人間可読な文字列を得るヘルパーを用意する。
// 思考フロー: (1) Todo/実行中/ブロック/完了など状態を洗い出しデータ構造を決める (2) 表示形式をmatchで分岐しformat!等で組み立てる (3) stateは消費してよいので値で受け取りStringを返す。
// 使用技術まとめ: データ付きenum、構造体バリアント、match式+パターンマッチング、format!マクロでの文字列生成。
// 参照: https://doc.rust-jp.rs/book-ja/ch06-01-defining-an-enum.html, https://doc.rust-jp.rs/book-ja/ch06-02-match.html
/// Debug/PartialEq/Eqをderiveしてテストやロギングをしやすくする。
#[derive(Debug, PartialEq, Eq)]
pub enum TaskState {
    /// まだ手を付けていないタスク。説明文を保持。
    Todo(String),
    /// 実行中タスク。構造体バリアントで名前と残り秒数を同時に保持。
    Running { name: String, remaining: u32 },
    /// ブロックされている理由文字列を持つ状態。
    Blocked(String),
    /// 完了状態。追加情報は不要。
    Done,
}

// match式で各バリアントを分岐し、フォーマット済みの文字列を組み立てる関数パターンマッチング。
// 参照: https://doc.rust-jp.rs/book-ja/ch06-02-match.html
/// TaskStateを受け取り、その状態を説明する文字列を返す。
pub fn describe_task(state: TaskState) -> String {
    // matchでstateのバリアントに応じ処理を分岐。moveするためstateの所有権はここで使い切る。
    match state {
        // Todoバリアントは中のStringをtextとして束縛し、format!で"TODO: ..."形式を作る。
        TaskState::Todo(text) => format!("TODO: {}", text),
        TaskState::Running { name, remaining } => {
            // 構造体バリアントはフィールド名で分解できる。running表示には名前と残り秒数を挿入。
            format!("Running {} ({}s left)", name, remaining)
        }
        // Blockedは理由を表示に組み込む。
        TaskState::Blocked(reason) => format!("Blocked: {}", reason),
        // Doneは固定文字列。String::fromでも良いがここではto_string()で新規Stringを生成。
        TaskState::Done => "Done".to_string(),
    }
}
