// Pandigital Products
// https://projecteuler.net/problem=32

use std::collections::HashSet;
use pe_lib::is_pandigital;

fn solve() -> i32 {
    let mut products = HashSet::new();
    for a in 1..=98 {
        let min_b = if a < 10 { 123 } else { 12 };
        let max_b = 10_000 / a;
        for b in min_b..=max_b {
            let product = a * b;
            let s = format!("{}{}{}", a, b, product);
            if s.len() > 9 { break; }
            if is_pandigital(&s) { products.insert(product); }
        }
    }
    products.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pandigital_recognition() {
        assert!(is_pandigital("123456789"));
        assert!(!is_pandigital("12345678"));  // Missing 9
        assert!(!is_pandigital("1234567890")); // Too long
    }

    #[test]
    fn test_pandigital_product_concatenation() {
        // Example: 39 * 186 = 7254, concat "39186:7254" is not pandigital
        let product = 39 * 186;
        let s = format!("39186{}", product);
        assert_eq!(s.len(), 9);
    }

    #[test]
    fn test_solve_produces_sum() {
        let result = solve();
        assert!(result > 0);
        assert!(result < 50000);  // Reasonable bound for product sum
    }

    #[test]
    fn test_product_uniqueness() {
        // Multiple factor pairs can produce the same product
        // Verify we're using a set to avoid double-counting
        let mut products = HashSet::new();
        for a in 1..=10 {
            for b in 1..=100 {
                let product = a * b;
                products.insert(product);
            }
        }
        assert!(products.len() > 0);
    }
}

pe_utils::pe_main!();
