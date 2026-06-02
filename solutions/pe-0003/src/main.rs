// Largest Prime Factor
// https://projecteuler.net/problem/3

use pe_lib::largest_prime_factor;

fn solve() -> u64 {
    largest_prime_factor(600_851_475_143)
}

pe_utils::pe_main!();
