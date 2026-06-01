// Coded Triangle Numbers
// https://projecteuler.net/problem=42

use std::fs;

fn is_triangle_word(s: &str) -> bool {
    let n: u32 = s.chars()
        .filter(|c| c.is_ascii_uppercase())
        .map(|c| c as u32 - 'A' as u32 + 1)
        .sum();
    let mut i = 1;
    let mut triangle_num = 1;
    while triangle_num < n {
        i += 1;
        triangle_num = i * (i + 1) / 2;
    }
    n == triangle_num
}

fn solve() -> usize {
    let content = fs::read_to_string("./data/0042_words.txt")
        .expect("failed to read ./data/0042_words.txt");
    let words: Vec<String> = content.split(',')
        .map(|s| s.trim_matches('"').to_string())
        .collect();
    words.iter().filter(|w| is_triangle_word(w)).count()
}

fn main() {
    pe_utils::run(42, solve);
}
