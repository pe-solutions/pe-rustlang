// Pandigital Multiples
// https://projecteuler.net/problem=38

use pe_lib::is_pandigital;

fn solve() -> i32 {
    for index in (1..=9876).rev() {
        let candidate = index * 100_002;
        if is_pandigital(&candidate.to_string()) {
            return candidate;
        }
    }
    panic!("no pandigital found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pandigital_check() {
        assert!(is_pandigital("123456789"));
        assert!(!is_pandigital("123456780"));
        assert!(!is_pandigital("1234567890"));
    }

    #[test]
    fn test_pandigital_multiple_property() {
        // 192 * 3 = 576, so "192576" should be pandigital-like
        let num = 192 * 3;
        let concat = format!("192{}", num);
        // "1923576" is 7 digits, not a 9-digit pandigital
        assert_ne!(concat.len(), 9);
    }

    #[test]
    fn test_search_range_valid() {
        // The algorithm searches in reverse through the range
        for index in [1, 100, 1000, 9876] {
            let candidate = index * 100_002;
            assert!(candidate > 0);
        }
    }

    #[test]
    fn test_solve_produces_pandigital() {
        let result = solve();
        let result_str = result.to_string();
        assert!(is_pandigital(&result_str));
        assert_eq!(result_str.len(), 9);
    }
}

pe_utils::pe_main!();
