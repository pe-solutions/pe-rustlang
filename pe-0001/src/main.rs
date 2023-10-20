// Multiples of 3 or 5
// https://projecteuler.net/problem=1

use std::time::Instant;
use gcd::Gcd;

fn main() {
    let start = Instant::now();
    //
    let answer: u64 = (1..=999).filter(|&a| a.gcd(3 * 5) > 1).sum();
    //
    let duration = start.elapsed();

    println!("\nProject Euler #1\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
