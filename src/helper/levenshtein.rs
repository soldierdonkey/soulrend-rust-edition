// Add this helper function to your utilities or directly inside your Registry impl block
pub fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let mut cache = vec![0; b_chars.len() + 1];
    for j in 0..=b_chars.len() { cache[j] = j; }

    for i in 1..=a_chars.len() {
        let mut prev = i;
        for j in 1..=b_chars.len() {
            let temp = cache[j];
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            cache[j] = std::cmp::min(std::cmp::min(cache[j] + 1, prev + 1), cache[j - 1] + cost);
            prev = temp;
        }
    }
    cache[b_chars.len()]
}