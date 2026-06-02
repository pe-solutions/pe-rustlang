// Summation of primes
// https://projecteuler.net/problem/10

use pe_lib::is_prime;

fn sum_primes_below_limit(limit: u64) -> u64 {
    let mut sum: u64 = 0;

    for num in 2..limit {
        if is_prime(num) {
            sum += num;
        }
    }

    sum
}

fn solve() -> u64 {
    sum_primes_below_limit(2_000_000)
}

pe_utils::pe_main!();
