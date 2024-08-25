// Powerful Digit Sum
// https://projecteuler.net/problem=56

use std::time::Instant;
use num_bigint::BigInt;
use num_traits::pow::Pow;

fn main() {
    let start = Instant::now();
    
    let max_digital_sum = (1..100)
        .flat_map(|a| (1..100).map(move |b| BigInt::from(a).pow(b as u32)))
        .map(|n| n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>())
        .max()
        .unwrap();

    let elapsed = start.elapsed();
    
    println!("\nProject Euler #56\nAnswer: {}", max_digital_sum);
    println!("Elapsed time: {} milliseconds.\n", elapsed.as_millis());
}

