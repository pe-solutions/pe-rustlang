// Distinct Primes Factors
// https://projecteuler.net/problem=47

use pe_lib::sieve_omega;

fn solve() -> usize {
    let limit = 200_000;
    let required_factors = 4;
    let omega = sieve_omega(limit);

    for i in 1..limit - required_factors {
        if (0..required_factors).all(|j| omega[i + j] == required_factors) {
            return i;
        }
    }

    0
}

pe_utils::pe_main!();
