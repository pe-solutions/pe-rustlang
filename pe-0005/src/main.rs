// Smallest Multiple
// https://projecteuler.net/problem/5

use std::time::Instant;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn main() {
    let start = Instant::now();
    
    let answer = (1..=20).fold(1, |acc, x| lcm(acc, x));
    
    let duration = start.elapsed();

    println!("\nProject Euler #5\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
