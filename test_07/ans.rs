pub fn sliding_window_sum(values: &[i64], window: usize) -> Vec<i64> {
    if window == 0 || window > values.len() {
        return Vec::new();
    }
    values
        .windows(window)
        .map(|chunk| chunk.iter().sum())
        .collect()
}
