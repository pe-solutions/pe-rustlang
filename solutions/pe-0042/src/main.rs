// Coded Triangle Numbers
// https://projecteuler.net/problem=42

use pe_lib::is_triangular;
use std::fs;

fn is_triangle_word(s: &str) -> bool {
    let n: u64 = s.chars()
        .filter(|c| c.is_ascii_uppercase())
        .map(|c| c as u64 - 'A' as u64 + 1)
        .sum();
    is_triangular(n)
}

fn solve() -> usize {
    let content = fs::read_to_string("./data/0042_words.txt")
        .expect("failed to read ./data/0042_words.txt");
    let words: Vec<String> = content.split(',')
        .map(|s| s.trim_matches('"').to_string())
        .collect();
    words.iter().filter(|w| is_triangle_word(w)).count()
}

pe_utils::pe_main!();
