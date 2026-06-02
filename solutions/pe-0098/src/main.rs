// Anagramic Squares
// https://projecteuler.net/problem=98

use std::collections::HashMap;

fn sorted_chars(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.iter().collect()
}

fn can_map(word: &str, num_str: &str, mapping: &mut HashMap<char, char>) -> bool {
    if word.len() != num_str.len() {
        return false;
    }
    let mut local_map = mapping.clone();
    for (w, d) in word.chars().zip(num_str.chars()) {
        if let Some(&existing) = local_map.get(&w) {
            if existing != d {
                return false;
            }
        } else {
            for (_, &v) in &local_map {
                if v == d {
                    return false;
                }
            }
            local_map.insert(w, d);
        }
    }
    *mapping = local_map;
    true
}

fn solve() -> u64 {
    let words_str = std::fs::read_to_string("data/0098_words.txt").unwrap_or_default();
    let cleaned = words_str.replace("\"", "");
    let words: Vec<&str> = cleaned.split(',').collect();

    let mut anagrams: HashMap<String, Vec<&str>> = HashMap::new();
    for word in &words {
        let key = sorted_chars(word);
        anagrams.entry(key).or_insert_with(Vec::new).push(word);
    }

    let mut max_value = 0u64;
    for (_, word_group) in anagrams {
        if word_group.len() < 2 {
            continue;
        }

        let min_len = word_group.iter().map(|w| w.len()).min().unwrap_or(0);
        let max_num = 10u64.pow(min_len as u32);
        let min_num = 10u64.pow((min_len - 1) as u32);

        let mut num = (min_num as f64).sqrt().ceil() as u64;
        while num * num < max_num {
            let square = num * num;
            let square_str = square.to_string();
            let mut valid = true;
            let mut mapping = HashMap::new();

            for word in &word_group {
                if !can_map(word, &square_str, &mut mapping) {
                    valid = false;
                    break;
                }
            }

            if valid {
                max_value = max_value.max(square);
            }
            num += 1;
        }
    }

    max_value
}

pe_utils::pe_main!();
