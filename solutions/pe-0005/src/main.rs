// Smallest Multiple
// https://projecteuler.net/problem/5

use pe_lib::gcd;

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn solve() -> u64 {
    (1..=20u64).fold(1, |acc, x| lcm(acc, x))
}

pe_utils::pe_main!();
