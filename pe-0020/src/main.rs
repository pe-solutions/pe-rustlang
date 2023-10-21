// Factorial Digit Sum
// https://projecteuler.net/problem=20

use num_bigint::BigUint;
use num_traits::One;

fn factorial(n: usize) -> BigUint {
    (1..=n).fold(BigUint::one(), |a, b| a * b)
}

fn main() {
    let start = std::time::Instant::now();

    let answer: u64 = factorial(100)
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum();

    let duration = start.elapsed();

    println!("\nProject Euler #20");
    println!("Answer: {}", answer);
    println!("Elapsed time: {} milliseconds.", duration.as_millis());
}
