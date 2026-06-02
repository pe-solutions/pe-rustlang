// Lychrel Numbers
// https://projecteuler.net/problem=55

use pe_lib::{reverse_digits_generic, is_palindrome_generic};

fn is_lychrel(mut number: u128) -> bool {
    for _ in 0..50 {
        number += reverse_digits_generic(number);

        if is_palindrome_generic(number) {
            return false;
        }
    }

    true
}

fn solve() -> u32 {
    let mut lychrel_total = 0;
    for candidate in 1..=10_000 {
        if is_lychrel(candidate as u128) { lychrel_total += 1; }
    }
    lychrel_total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_lychrel_palindromes() {
        // Single digit numbers are palindromes immediately
        assert!(!is_lychrel(1));
        assert!(!is_lychrel(5));
        assert!(!is_lychrel(9));
    }

    #[test]
    fn test_non_lychrel_19() {
        // 19 + 91 = 110, 110 + 011 = 121 (palindrome)
        assert!(!is_lychrel(19));
    }

    #[test]
    fn test_non_lychrel_89() {
        // 89 converges to a palindrome within 50 iterations
        assert!(!is_lychrel(89));
    }

    #[test]
    fn test_lychrel_detection() {
        // Most numbers should not be Lychrel
        // Test a small sample to ensure variety
        let mut non_lychrels = 0;
        for n in 1..=100 {
            if !is_lychrel(n as u128) {
                non_lychrels += 1;
            }
        }
        assert!(non_lychrels > 50); // Most should be non-Lychrel
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > 0);
        assert!(result < 10_000); // Lychrels are rare
    }
}

pe_utils::pe_main!();
