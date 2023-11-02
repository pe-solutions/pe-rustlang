// Consecutive prime sum
// https://projecteuler.net/problem=50


use std::vec::Vec;

fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=((n as f64).sqrt() as i64) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let start = std::time::Instant::now();

    const LIMIT: i64 = 1_000_000;
    
    let primes: Vec<i64> = (2..LIMIT).filter(|&x| is_prime(x)).collect();
    
    let mut max_sum = 0;
    let mut max_length = 0;
    
    for i in 0..primes.len() {
        let mut sum = primes[i];
        
        for j in i + 1..primes.len() {
            sum += primes[j];
            
            if is_prime(sum) {
                let length = j - i + 1;
                
                if length > max_length {
                    max_length = length;
                    max_sum = sum;
                }
            }
            
            if sum >= LIMIT {
                break;
            }
        }
    }
    
    let duration = start.elapsed();
    
    println!("\nProject Euler #50\nAnswer:{}", max_sum);
    println!("Elapsed time: {} milliseconds.", duration.as_millis());
}
