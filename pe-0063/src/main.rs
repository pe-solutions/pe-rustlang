// Powerful Digit Counts
// https://projecteuler.net/problem=63

use std::time::Instant;
use std::f64;

fn main() {
    let start = Instant::now();
    
    let mut total_count = 0;

    for i in 1..10 {
        let log_i = (i as f64).log10();
        let count = (1.0 / (1.0 - log_i)).floor() as i32;
        
        total_count += count;
    }
    
    let duration = start.elapsed();
    
    println!("\nProject Euler #63\nAnswer: {}", total_count);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
