// Square Digit Chains
// https://projecteuler.net/problem=92

use std::collections::HashMap;

// Computes the sum of the squares of the digits of `n`.
fn sum_of_digit_squares(mut n: u64) -> u64 {
    let mut sum = 0;
    
    while n > 0 {
        let digit = n % 10;
        sum += digit * digit;
        n /= 10;
    }
    
    sum
}

// Determines if the number `n` eventually ends up at 89 or 1,
// using memoization to cache previously computed results.
fn ends_in_89(n: u64, memo: &mut HashMap<u64, bool>) -> bool {
    let mut current = n;
    while current != 1 && current != 89 {
        current = sum_of_digit_squares(current);
    }
    
    let result = current == 89;
    
    memo.insert(n, result);
    
    result
}

fn main() {
    let start = std::time::Instant::now();
    
    let limit = 10_000_000;
    
    let mut count = 0;
    
    let mut memo = HashMap::new();

    // Count how many numbers below `limit` eventually end up at 89.
    for i in 1..limit {
        if ends_in_89(i, &mut memo) {
            count += 1;
        }
    }
    
    let duration = start.elapsed();
    
    println!("\nProject Euler #92\nAnswer: {}", count);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
