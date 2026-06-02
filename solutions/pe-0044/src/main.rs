// Pentagon Numbers
// https://projecteuler.net/problem=44

use pe_lib::is_pentagonal as pe_is_pentagonal;

fn is_pentagonal(n: u32) -> bool {
    pe_is_pentagonal(n as u64)
}

fn solve() -> u32 {
    let mut a: u32;
    let mut b: u32;
    for i in 1..2500 {
        for j in 1..i {
            a = i * (3 * i - 1) / 2;
            b = j * (3 * j - 1) / 2;
            if is_pentagonal(a - b) && is_pentagonal(a + b) {
                return a - b;
            }
        }
    }
    panic!("no answer found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pentagonal_formula() {
        // Known pentagon numbers: 1, 5, 12, 22, 35, 51
        let p1 = 1 * (3 * 1 - 1) / 2;
        let p2 = 2 * (3 * 2 - 1) / 2;
        let p3 = 3 * (3 * 3 - 1) / 2;
        assert_eq!(p1, 1);
        assert_eq!(p2, 5);
        assert_eq!(p3, 12);
    }

    #[test]
    fn test_is_pentagonal_known_values() {
        assert!(is_pentagonal(1));
        assert!(is_pentagonal(5));
        assert!(is_pentagonal(12));
        assert!(is_pentagonal(22));
        assert!(!is_pentagonal(10));
        assert!(!is_pentagonal(20));
    }

    #[test]
    fn test_pentagonal_difference_sum() {
        // Test that we correctly identify when both sum and difference are pentagonal
        let a = 12u32;
        let b = 5u32;
        // 12 + 5 = 17 (not pentagonal), 12 - 5 = 7 (not pentagonal)
        assert!(!is_pentagonal(a + b) || !is_pentagonal(a - b));
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > 0);
    }
}

pe_utils::pe_main!();
