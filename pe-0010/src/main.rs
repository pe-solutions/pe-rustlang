// Summation of primes
// https://projecteuler.net/problem/10

use std::time::Instant;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn sum_primes_below_limit(limit: u64) -> u64 {
    let mut sum: u64 = 0;

    for num in 2..limit {
        if is_prime(num) {
            sum += num;
        }
    }

    sum
}

fn main() {
    let start = Instant::now();
    
    let limit = 2_000_000;
    let answer = sum_primes_below_limit(limit);
    
    let duration = start.elapsed();

    println!("\nProject Euler #10\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
