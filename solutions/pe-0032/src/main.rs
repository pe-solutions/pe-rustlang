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

pe_utils::pe_main!();
