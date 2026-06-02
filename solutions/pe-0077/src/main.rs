// Prime Summations
// https://projecteuler.net/problem=77

use pe_lib::sieve_primes;

fn solve() -> u64 {
    let primes = sieve_primes(100);
    let mut ways = vec![0; 101];
    ways[0] = 1;
    
    for &p in &primes {
        for i in p..101 {
            ways[i] += ways[i - p];
        }
    }
    
    for i in 1..101 {
        if ways[i] > 5000 {
            return i as u64;
        }
    }
    0
}

pe_utils::pe_main!();
