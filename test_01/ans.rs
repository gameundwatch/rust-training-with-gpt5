pub fn build_greeting(name: &str, times: usize) -> String {
    let mut result = String::new();
    for idx in 0..times {
        if idx > 0 {
            result.push('\n');
        }
        result.push_str(&format!("[{}] Hello, {}!", idx + 1, name));
    }
    result
}
