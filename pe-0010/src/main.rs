// Summation of primes
// https://projecteuler.net/problem=10

use std::time::Instant;
use num_prime::nt_funcs::{is_prime64};

fn sum_primes_below_limit(limit: u64) -> u64 {
    let mut sum: u64 = 0;

    for num in 2..limit {
        if is_prime64(num) {
            sum += num as u64;
        }
    }

    sum
}

fn main() {
    let start = Instant::now();
    //
    let limit = 2_000_000;

    let answer = sum_primes_below_limit(limit);
    
    //
    let duration = start.elapsed();

    println!("\nProject Euler #10\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis()); 
}
