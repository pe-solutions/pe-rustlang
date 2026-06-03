// Largest Product in a Series
// https://projecteuler.net/problem=8

use std::fs;

fn solve() -> i64 {
    let data = fs::read_to_string("data/0008_digits.txt")
        .expect("failed to read data/0008_digits.txt")
        .trim()
        .to_string();

    let digits: Vec<i64> = data.chars().map(|c| c.to_digit(10).unwrap() as i64).collect();
    let mut answer = 0;
    for start in 0..(digits.len() - 12) {
        let product = digits[start..(start + 13)].iter().product();
        if product > answer {
            answer = product;
        }
    }
    answer
}

pe_utils::pe_main!();
