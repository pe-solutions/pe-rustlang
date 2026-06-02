// Double-base Palindromes
// https://projecteuler.net/problem=36

use pe_lib::is_palindrome_str as is_palindrome;

fn solve() -> i32 {
    let mut answer = 0;
    for i in 1..1_000_000 {
        let decimal_str = i.to_string();
        let binary_str = format!("{:b}", i);
        if is_palindrome(&decimal_str) && is_palindrome(&binary_str) {
            answer += i;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_strings() {
        assert!(is_palindrome("121"));
        assert!(is_palindrome("9"));
        assert!(!is_palindrome("123"));
        assert!(!is_palindrome("1210"));
    }

    #[test]
    fn test_binary_representation() {
        let num = 9;
        let binary = format!("{:b}", num);
        assert_eq!(binary, "1001");  // 9 in binary is 1001 (palindrome!)
        assert!(is_palindrome(&binary));
    }

    #[test]
    fn test_double_base_palindrome_9() {
        // 9 is palindrome in both decimal (9) and binary (1001)
        let decimal_str = "9";
        let binary_str = "1001";
        assert!(is_palindrome(decimal_str));
        assert!(is_palindrome(binary_str));
    }

    #[test]
    fn test_solve_produces_output() {
        let result = solve();
        assert!(result > 0);
        // At least includes 9 (double-base palindrome)
        assert!(result >= 9);
    }
}

pe_utils::pe_main!();
