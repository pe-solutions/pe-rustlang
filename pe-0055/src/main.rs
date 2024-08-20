// Lychrel Numbers
// https://projecteuler.net/problem=55

use std::time::Instant;

fn reverse_digits(mut number: u128) -> u128 {
    let mut reversed = 0;
    
    while number > 0 {
        reversed = reversed * 10 + (number % 10);
        number /= 10;
    }
    
    reversed
}

fn is_palindrome(number: u128) -> bool {
    number == reverse_digits(number)
}

fn is_lychrel(mut number: u128) -> bool {
    for _ in 0..50 {
        number += reverse_digits(number);
        
        if is_palindrome(number) {
            return false;
        }
    }
    
    true
}

fn main() {
    let start = Instant::now();
    
    let mut lychrel_total = 0;
    
    for candidate in 1..=10_000 {
        if is_lychrel(candidate as u128) {
            lychrel_total += 1;
        }
    }
    
    let elapsed = start.elapsed();
    
    println!("\nProject Euler #55\nAnswer: {}", lychrel_total);
    println!("Elapsed time: {} milliseconds.\n", elapsed.as_millis());
}
