// Names scores
// https://projecteuler.net/problem=22

use std::fs;
use std::io;

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

fn main() -> io::Result<()> {
    let start = std::time::Instant::now();

    let mut names: Vec<String> = fs::read_to_string("./data/0022_names.txt")?
        .split(',')
        .map(|s| s.trim_matches('"').to_string())
        .collect();

    names.sort();

    let answer = all_name_score(&names);

    let duration = start.elapsed();

    println!("\nProject Euler #22\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());

    Ok(())
}
