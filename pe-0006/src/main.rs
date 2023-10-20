// Sum Square Difference
// https://projecteuler.net/problem=6

use std::time::Instant;

fn main() {
    let start = Instant::now();
    //
    
    let sum_of_squares: i64 = (1..=100).sum::<i64>().pow(2);
    let square_of_sums: i64 = (1..=100).map(|x: i64| x.pow(2)).sum();
    
    let answer = sum_of_squares - square_of_sums;
    
    //
    let duration = start.elapsed();

    println!("\nProject Euler #6\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
