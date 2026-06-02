// Maximum Path Sum I
// https://projecteuler.net/problem=18

use std::fs;

fn folding_sum(numbers: &mut Vec<Vec<u64>>) -> u64 {
    for i in (0..numbers.len() - 1).rev() {
        for j in 0..numbers[i].len() {
            numbers[i][j] += numbers[i + 1][j].max(numbers[i + 1][j + 1]);
        }
    }
    numbers[0][0]
}

fn solve() -> u64 {
    let content = fs::read_to_string("data/0018_triangle.txt").expect("failed to read data/0018_triangle.txt");
    let mut triangle: Vec<Vec<u64>> = content.lines()
        .map(|line| line.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();
    folding_sum(&mut triangle)
}

pe_utils::pe_main!();
