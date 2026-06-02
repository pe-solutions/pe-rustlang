// Nth Digit of Reciprocals
// https://projecteuler.net/problem=820

use pe_lib::mod_pow;

fn calculate_nth_digit_sum(n: u64) -> u64 {
    (1..=n)
        .map(|i| mod_pow(10, n - 1, i) * 10 / i)
        .sum()
}

fn solve() -> u64 {
    calculate_nth_digit_sum(10_000_000)
}

pe_utils::pe_main!();
