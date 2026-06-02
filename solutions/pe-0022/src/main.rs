// Names scores
// https://projecteuler.net/problem=22

use pe_lib::read_file_to_string;

fn name_value(name: &str) -> u32 {
    name.chars().map(|c| c as u32 - 'A' as u32 + 1).sum()
}

fn score_at(index: usize, names: &Vec<String>) -> u32 {
    (index + 1) as u32 * name_value(&names[index])
}

fn all_name_score(names: &Vec<String>) -> u32 {
    let mut sum = 0;
    for i in 0..names.len() {
        sum += score_at(i, names);
    }
    sum
}

fn solve() -> u32 {
    let mut names: Vec<String> = read_file_to_string("./data/0022_names.txt")
        .expect("failed to read ./data/0022_names.txt")
        .split(',')
        .map(|s| s.trim_matches('"').to_string())
        .collect();
    names.sort();
    all_name_score(&names)
}

pe_utils::pe_main!();
