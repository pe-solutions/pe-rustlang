// Prime Power Triples
// https://projecteuler.net/problem=87

use primes::is_prime;

const LIMIT: u64 = 50_000_000;

fn solve() -> usize {
    let mut results = std::collections::HashSet::new();
    let primes: Vec<u64> = (2..=7_071).filter(|&x| is_prime(x)).collect();
    for p2 in primes.iter().map(|&p| p * p).filter(|&x| x < LIMIT) {
        for p3 in primes.iter().map(|&p| p * p * p).filter(|&x| p2 + x < LIMIT) {
            for p4 in primes.iter().map(|&p| p * p * p * p).filter(|&x| p2 + p3 + x < LIMIT) {
                results.insert(p2 + p3 + p4);
            }
        }
    }
    results.len()
}

pe_utils::pe_main!();
