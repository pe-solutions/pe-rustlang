// Factorial Digit Sum
// https://projecteuler.net/problem=20

use pe_lib::{factorial, digit_sum};

fn solve() -> u64 {
    let fact = factorial(100);
    fact.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum()
}

pe_utils::pe_main!();
