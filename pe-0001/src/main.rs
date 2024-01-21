// Multiples of 3 or 5
// https://projecteuler.net/problem=1

use std::time::Instant;

fn solve(multiples: &[i32], limit: i32) -> i32 {
    (1..limit)
        .filter(|&n| multiples.iter().any(|&m| n % m == 0))
        .sum()
}

fn main() {
    let start = Instant::now();
    //
    let answer = solve(&[3, 5], 1000);
    //
    let duration = start.elapsed();
    
    println!("\nProject Euler #1\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
