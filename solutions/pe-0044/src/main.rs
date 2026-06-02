// Pentagon Numbers
// https://projecteuler.net/problem=44

use pe_lib::{is_pentagonal as pe_is_pentagonal, isqrt, is_perfect_square};

fn is_pentagonal(n: u32) -> bool {
    pe_is_pentagonal(n as u64)
}

fn solve() -> u32 {
    let mut a: u32;
    let mut b: u32;
    for i in 1..2500 {
        for j in 1..i {
            a = i * (3 * i - 1) / 2;
            b = j * (3 * j - 1) / 2;
            if is_pentagonal(a - b) && is_pentagonal(a + b) {
                return a - b;
            }
        }
    }
    panic!("no answer found")
}

pe_utils::pe_main!();
