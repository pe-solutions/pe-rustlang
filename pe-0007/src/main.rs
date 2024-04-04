// 10001st Prime
// https://projecteuler.net/problem=7

use std::time::Instant;

fn main() {
    let start = Instant::now();
    
    /// Pari/GP calculation: 
    /// a(n) = n*(log(n) + log(log(n)))
    /// round(a(10001)) = 114319
    
    const UPPER_LIMIT: usize = 114_319;

    let mut primes = vec![true; UPPER_LIMIT + 1];
    let max_prime = (UPPER_LIMIT as f64).sqrt() as usize;

    sieve_of_eratosthenes(&mut primes, UPPER_LIMIT, max_prime);

    match find_nth_prime(&primes, 10001) {
        Ok(answer) => println!("\nProject Euler #7\nAnswer: {}", answer),
        Err(err) => eprintln!("Error: {}", err),
    }
    
    let duration = start.elapsed();
    
    //
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}

fn sieve_of_eratosthenes(primes: &mut Vec<bool>, upper_limit: usize, max_prime: usize) {
    for number in 2..=max_prime {
        if primes[number] {
            for multiple in (number * number..=upper_limit).step_by(number) {
                primes[multiple] = false;
            }
        }
    }
}

fn find_nth_prime(primes: &[bool], n: usize) -> Result<usize, String> {
    let mut count = 0;
    
    for (i, &is_prime) in primes.iter().enumerate() {
        if is_prime {
            count += 1;
            
            if count == n {
                return Ok(i);
            }
        }
    }

    Err("Not enough primes found.".to_string())
}
