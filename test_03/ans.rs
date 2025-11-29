use std::collections::HashMap;

pub fn word_tally(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for raw in text.split(|c: char| !c.is_alphanumeric()) {
        if raw.is_empty() {
            continue;
        }
        let word = raw.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}
