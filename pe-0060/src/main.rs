// Prime Pair Sets
// https://projecteuler.net/problem=60

use std::time::Instant;

mod prime_utils;
use crate::prime_utils::find_prime_set;

fn main() {
    let start = Instant::now();
    
    if let Some(result) = find_prime_set() {
        println!("\nProject Euler #60\nAnswer: {}", result.iter().sum::<u64>());
    }
    
    let duration = start.elapsed();
    
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
