// Sum Square Difference
// https://projecteuler.net/problem/6

use std::time::Instant;

fn sum_square_difference(n: i64) -> i64 {
    let sum_of_squares: i64 = (1..=n).map(|x| x.pow(2)).sum();
    let square_of_sums: i64 = (1..=n).sum::<i64>().pow(2);
    
    square_of_sums - sum_of_squares
}

fn main() {
    let start = Instant::now();
    
    let n: i64 = 100;
    let answer = sum_square_difference(n);
    
    let duration = start.elapsed();

    println!("\nProject Euler #6\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
