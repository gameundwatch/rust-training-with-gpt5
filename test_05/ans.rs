// 方針: タスクの状態をenumで表し、画面表示用の文字列に変換する共通関数を用意する。
// 思考フロー: (1) 状態ごとに必要な情報(文字列/残り秒数など)を洗い出しenumを設計 (2) マッチングで各状態の表示形式を決める (3) 所有権を消費しても良いか確認しstateを値で受け取る。
// 使用技術まとめ: データ保持付きenum、構造体風バリアント、match式でのパターンマッチングとformat!マクロ。
// タスク状態を列挙型の各バリアントに保持するデータを変えて表現する状態遷移モデル。
// 参照: https://doc.rust-jp.rs/book-ja/ch06-01-defining-an-enum.html
#[derive(Debug, PartialEq, Eq)]
pub enum TaskState {
    Todo(String),
    Running { name: String, remaining: u32 },
    Blocked(String),
    Done,
}

// match式で各バリアントを分岐し、フォーマット済みの文字列を組み立てる関数パターンマッチング。
// 参照: https://doc.rust-jp.rs/book-ja/ch06-02-match.html
pub fn describe_task(state: TaskState) -> String {
    match state {
        TaskState::Todo(text) => format!("TODO: {}", text),
        TaskState::Running { name, remaining } => {
            format!("Running {} ({}s left)", name, remaining)
        }
        TaskState::Blocked(reason) => format!("Blocked: {}", reason),
        TaskState::Done => "Done".to_string(),
    }
}
