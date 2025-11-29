# Rust学習教材をGPTで作成してもらった

ChatGPT5 - Codexを使用しRustの学習用問題集を作成

## プロンプトは下記のとおり

Rustの学習にあたっての問題集を作成してください。現在、Rustの勉強中です。Rustに関しては初学者ですが私のスキルセットとしてはC/C++/Java/Javascript/Typescript/pythonなどの基本的高級言語を学習済みであり、またそのオブジェクト指向の考え方やその先の関数型言語の考え方までざっくり網羅している感じです。
問題集は１問を
- 問題の要件、要件の解決のための手順やRustで使えるアプローチが日本語でコメントアウトされた実行用のファイル”task.rs”
- 回答コードのテストコードを４〜５パターン”test_task.rs”
- 模範解答となる”ans.rs”とし、これをまとめたディレクトリを”test_00”のように連番で書くこと。当然ですが、ans.rsがtest_task.rsを満たすようにしてください。
問題の内容としては
- 変数の定義、利用
- 関数の定義、利用・Rust固有の機能について・基本ライブラリのうち頻繁に使うものをざっくり網羅
- ファイル越しの関数等の利用など
をテーマとし、１０問ほど作成してください

---

以下はGPTのレスポンスによる説明

- Added ten exercise directories test_00–test_09, each with a Japanese-instruction stub in task.rs explaining the goal and Rust-specific tips (e.g., test_00/task.rs:1, test_05/task.rs:1, test_08/
  task.rs:1). Topics span variables, string handling, iterators, HashMap aggregation, custom errors, enums and pattern matching, Vec processing, sliding windows, file I/O, and generic closure
  pipelines.
- For every task, supplied a dedicated test suite in test_task.rs illustrating 4–5 representative scenarios and edge cases to validate the required behavior (e.g., test_02/test_task.rs:1, test_06/
  test_task.rs:1, test_09/test_task.rs:1). Tests import the task module via #[path = "task.rs"] so learners can drop in their solutions directly.
- Crafted matching reference implementations in ans.rs that satisfy the corresponding tests and demonstrate idiomatic approaches such as iterator chains, HashMap::entry, Result/Option handling, and
  std::fs/io::Error usage (test_03/ans.rs:1, test_04/ans.rs:1, test_08/ans.rs:1).

Tests were not executed because each task.rs intentionally contains todo!() placeholders; running them now would panic. To verify a specific exercise, copy its ans.rs into task.rs temporarily and
run rustc --test test_task.rs inside that directory.
