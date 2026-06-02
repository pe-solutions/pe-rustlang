// Counting Fractions
// https://projecteuler.net/problem=72

use pe_lib::totient_sieve;

fn solve() -> u64 {
    let phi = totient_sieve(1_000_000);
    phi[2..].iter().map(|&x| x as u64).sum()
}

pe_utils::pe_main!();
