// Largest Palindrome Product
// https://projecteuler.net/problem=4

use pe_lib::is_palindrome_num;

fn is_palindrome(n: i32) -> bool {
    is_palindrome_num(n as u64)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(11));
        assert!(is_palindrome(121));
        assert!(is_palindrome(9009));
        assert!(!is_palindrome(12));
        assert!(!is_palindrome(123));
    }

    #[test]
    fn test_find_largest_small_range() {
        let result = find_largest_palindrome_product(10..=20);
        assert!(result.is_some());
        let pal = result.unwrap();
        assert!(is_palindrome(pal));
    }

    #[test]
    fn test_palindrome_product_property() {
        for a in 10..20 {
            for b in 10..20 {
                let product = a * b;
                if is_palindrome(product) {
                    assert_eq!(product, (product.to_string().chars().rev().collect::<String>()).parse::<i32>().unwrap());
                }
            }
        }
    }

    #[test]
    fn test_solve_produces_valid_palindrome() {
        let result = solve();
        assert!(is_palindrome(result));
        assert!(result > 0);
    }
}

pe_utils::pe_main!();
