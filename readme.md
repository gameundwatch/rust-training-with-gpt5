# Rust学習教材をGPTで作成してもらった

ChatGPT5 - Codexを使用しRustの学習用問題集を作成

問題と回答はtask.rsを書き換える、テストコードの実行には下記の方法で
```
run rustc --test test_task.rs
```

学習中に手元で素早くPASS/FAILを確認したい場合は、`test_task.rs`を通常のバイナリとして
コンパイルすればコンソールに各ケースの結果が表示されます（正答は表示しません）。

```
rustc test_task.rs && ./test_task   # Windowsはtest_task.exe
```
