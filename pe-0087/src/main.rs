// Prime Power Triples
// https://projecteuler.net/problem=87

use std::time::Instant;
use std::collections::HashSet;
use primes::is_prime;

const LIMIT: u64 = 50_000_000;

fn main() {
    let start = Instant::now();
    
    let mut results = HashSet::new();
    
    // primes up to sqrt(50M)
    let primes: Vec<u64> = (2..=7_071).filter(|&x| is_prime(x)).collect();

    for p2 in primes.iter().map(|&p| p * p).filter(|&x| x < LIMIT) {
        for p3 in primes.iter().map(|&p| p * p * p).filter(|&x| p2 + x < LIMIT) {
            for p4 in primes.iter().map(|&p| p * p * p * p).filter(|&x| p2 + p3 + x < LIMIT) {
                results.insert(p2 + p3 + p4);
            }
        }
    }
    
    let duration = start.elapsed();

    println!("\nProject Euler #87\nAnswer: {}", results.len());
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
