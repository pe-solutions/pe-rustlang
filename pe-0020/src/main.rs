// Factorial Digit Sum
// https://projecteuler.net/problem=20

use num_bigint::BigUint;
use num_traits::One;

fn factorial(n: usize) -> BigUint {
    (1..=n).fold(BigUint::one(), |a, b| a * b)
}

fn solve() -> u64 {
    factorial(100)
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum()
}

fn main() {
    pe_utils::run(20, solve);
}
