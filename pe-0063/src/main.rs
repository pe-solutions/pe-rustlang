// Powerful Digit Counts
// https://projecteuler.net/problem=63

use std::f64;

fn solve() -> i32 {
    let mut total_count = 0;
    for i in 1..10 {
        let log_i = (i as f64).log10();
        total_count += (1.0 / (1.0 - log_i)).floor() as i32;
    }
    total_count
}

fn main() {
    pe_utils::run(63, solve);
}
