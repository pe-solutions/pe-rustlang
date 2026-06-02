// Spiral Primes
// https://projecteuler.net/problem=58

use pe_lib::is_prime;

fn solve() -> u64 {
    let mut n = 1u64;
    let mut side_length = 1u64;
    let mut primes_on_diagonal = 0u64;
    let mut numbers_on_diagonal = 1u64;

    loop {
        side_length += 2;
        let step = side_length - 1;

        // Four corners of the new square
        for _ in 0..4 {
            n += step;
            numbers_on_diagonal += 1;
            if is_prime(n) {
                primes_on_diagonal += 1;
            }
        }

        if primes_on_diagonal * 100 < numbers_on_diagonal * 10 {
            return side_length;
        }
    }
}

pe_utils::pe_main!();
