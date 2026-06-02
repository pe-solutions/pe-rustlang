// Triangular, Pentagonal, and Hexagonal
// https://projecteuler.net/problem=45

use pe_lib::{is_pentagonal, is_hexagonal};

fn solve() -> u64 {
    let mut n = 2u64;
    loop {
        let t = n * (n + 1) / 2;
        if is_pentagonal(t) && is_hexagonal(t) {
            return t;
        }
        n += 1;
    }
}

pe_utils::pe_main!();
