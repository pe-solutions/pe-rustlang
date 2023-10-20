// 10001st Prime
// https://projecteuler.net/problem=7

use std::time::Instant;
use num_prime::nt_funcs::{nth_prime};

fn main() {
    let start = Instant::now();
    //
        
    let answer = nth_prime(10_001);
    
    //
    let duration = start.elapsed();

    println!("\nProject Euler #7\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
