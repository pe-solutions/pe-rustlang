// Largest Prime Factor
// https://projecteuler.net/problem=3

use std::time::Instant;

fn largest_prime_factor(mut n: u64) -> u64 {
    let mut largest_factor = 0;

    while n % 2 == 0 {
        largest_factor = 2;
        n /= 2;
    }

    let mut i = 3;
    
    while i * i <= n {
        while n % i == 0 {
            largest_factor = i;
            n /= i;
        }
        i += 2;
    }

    if n > 2 {
        largest_factor = n;
    }

    largest_factor
}


fn main() {
    let start = Instant::now();
    //
    
    let answer: u64 = largest_prime_factor(600_851_475_143);

    //
    let duration = start.elapsed();

    println!("\nProject Euler #2\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis()); 
}
