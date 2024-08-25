// Powerful Digit Sum
// https://projecteuler.net/problem=56

use std::time::Instant;
use num_bigint::BigInt;
use num_traits::pow::Pow;

fn main() {
    let start = Instant::now();

    // Generate range of values for a and b
    let range_a = 1..100;
    let range_b = 1..100;

    // Compute maximum digital sum
    let max_digital_sum = range_a
        .flat_map(|a| range_b.clone().map(move |b| (a, b)))
        .map(|(a, b)| BigInt::from(a).pow(b as u32))  // Cast `b` to `u32` and pass by value
        .map(|n: BigInt| n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>())
        .max()
        .unwrap();

    let elapsed = start.elapsed();

    println!("\nProject Euler #56\nAnswer: {}", max_digital_sum);
    println!("Elapsed time: {} milliseconds.\n", elapsed.as_millis());
}
