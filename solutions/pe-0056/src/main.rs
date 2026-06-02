// Powerful Digit Sum
// https://projecteuler.net/problem=56

use num_bigint::BigInt;
use num_traits::pow::Pow;

fn solve() -> u32 {
    (1..100)
        .flat_map(|a| (1..100).map(move |b| BigInt::from(a).pow(b as u32)))
        .map(|n| n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>())
        .max()
        .unwrap()
}

pe_utils::pe_main!();
