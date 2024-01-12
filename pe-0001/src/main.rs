// Multiples of 3 or 5
// https://projecteuler.net/problem=1

use std::time::Instant;

fn gcd(m: u64, n: u64) -> u64 {
   if m == 0 {
      n
   } else {
      gcd(n % m, m)
   }
}

fn main() {
    let start = Instant::now();
    //
    let answer: u64 = (1u64..=999).filter(|&a| gcd(3u64 * 5u64, a) > 1u64).sum();
    //
    let duration = start.elapsed();

    println!("\nProject Euler #1\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
