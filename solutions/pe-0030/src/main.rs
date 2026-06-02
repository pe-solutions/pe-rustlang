// Digit fifth powers
// https://projecteuler.net/problem=30

use pe_lib::digits;

fn solve() -> u32 {
    (2..10u32.pow(6))
        .filter(|&i| {
            let digit_sum: u32 = digits(i as u64).iter().map(|&d| (d as u32).pow(5)).sum();
            i == digit_sum
        })
        .sum()
}

pe_utils::pe_main!();
