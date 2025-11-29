pub fn normalize_and_sort(items: Vec<String>) -> Vec<String> {
    let mut cleaned: Vec<String> = items
        .into_iter()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();
    cleaned.sort();
    cleaned.dedup();
    cleaned
}
