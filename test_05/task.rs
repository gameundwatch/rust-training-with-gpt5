// 問題: 列挙型とパターンマッチングを使って状態を文字列化してください。
// - タスクの状態を表す `TaskState` を定義しました。各バリアントから情報を取り出して整形します。
// - 出力フォーマット:
//   * `TaskState::Todo(text)` => "TODO: {text}"
//   * `TaskState::Running { name, remaining }` => "Running {name} ({remaining}s left)"
//   * `TaskState::Blocked(reason)` => "Blocked: {reason}"
//   * `TaskState::Done` => "Done"
// - `match` を使ってすべてのバリアントを網羅的に処理してください。
#[derive(Debug, PartialEq, Eq)]
pub enum TaskState {
    Todo(String),
    Running { name: String, remaining: u32 },
    Blocked(String),
    Done,
}

pub fn describe_task(state: TaskState) -> String {
    todo!("ここに処理を書いてください")
}
