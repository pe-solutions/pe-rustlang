// Smallest Multiple
// https://projecteuler.net/problem=5

use std::time::Instant;
use num::integer::lcm;


fn main() {
    let start = Instant::now();
    //
    
    let answer = (1..=20).fold(1, |acc, x| lcm(acc, x));
    
    //
    let duration = start.elapsed();

    println!("\nProject Euler #5\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis()); 
}
