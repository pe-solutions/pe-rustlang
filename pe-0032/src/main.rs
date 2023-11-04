// Pandigital Products
// https://projecteuler.net/problem=32

use std::collections::HashSet;

fn is_pandigital(s: &str) -> bool {
    if s.len() != 9 || s.chars().any(|c| c == '0' || s.chars().filter(|&d| d == c).count() != 1) {
        return false;
    }

    true
}

fn main() {
    let start = std::time::Instant::now();
    
    let mut products = HashSet::new();

    for a in 1..=98 {
        let min_b = if a < 10 { 123 } else { 12 };
        let max_b = 10_000 / a;

        for b in min_b..=max_b {
            let product = a * b;
            let s = format!("{}{}{}", a, b, product);

            if s.len() > 9 {
                break;
            }

            if is_pandigital(&s) {
                products.insert(product);
            }
        }
    }

    let answer: i32 = products.iter().sum();

    let duration = start.elapsed();

    println!("\nProject Euler #32\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
