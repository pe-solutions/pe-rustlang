// Distinct Powers
// https://projecteuler.net/problem=29

use num_bigint::BigUint;

fn main() {
    let start = std::time::Instant::now();
    
    let mut arr = Vec::new();

    for i in 2..=100 {
        for j in 2..=100 {
            arr.push(BigUint::from(i as u32).pow(j));
        }
    }

    arr.sort();
    arr.dedup();

    let answer = arr.len();

    let duration = start.elapsed();
    
    println!("\nProject Euler #29\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
