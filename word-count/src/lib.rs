use std::collections::HashMap;

pub fn word_count(phrase: &str) -> HashMap<String, u32> {
    let sanitize = |c: &char| c.is_alphanumeric() || c.is_whitespace();
    let phrase: String = phrase.chars().filter(sanitize).collect();

    let mut map: HashMap<String, u32> = HashMap::new();
    for word in phrase.split_whitespace() {
        let e = map.entry(word.to_lowercase()).or_insert(0);
        *e += 1;
    }
    map
}
