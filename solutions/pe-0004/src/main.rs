// Largest Palindrome Product
// https://projecteuler.net/problem=4

fn is_palindrome(n: i32) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

fn find_largest_palindrome_product(range: std::ops::RangeInclusive<i32>) -> Option<i32> {
    let mut max_palindrome = 0;

    for a in range.clone() {
        for b in range.clone() {
            let product = a * b;
            if is_palindrome(product) && product > max_palindrome {
                max_palindrome = product;
            }
        }
    }

    if max_palindrome > 0 {
        Some(max_palindrome)
    } else {
        None
    }
}

fn solve() -> i32 {
    find_largest_palindrome_product(899..=999).expect("no palindrome found")
}

pe_utils::pe_main!();
