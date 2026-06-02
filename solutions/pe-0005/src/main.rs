// Smallest Multiple
// https://projecteuler.net/problem/5

use pe_lib::gcd;

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn solve() -> u64 {
    (1..=20u64).fold(1, |acc, x| lcm(acc, x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(21, 14), 7);
        assert_eq!(gcd(7, 11), 1);
    }

    #[test]
    fn test_lcm_basic() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(5, 7), 35);
    }

    #[test]
    fn test_lcm_with_multiples() {
        // LCM of coprime numbers is their product
        assert_eq!(lcm(13, 17), 13 * 17);
        // LCM of a and b where a divides b
        assert_eq!(lcm(5, 15), 15);
    }

    #[test]
    fn test_lcm_fold_property() {
        // Test small case: LCM of 1..4 should be 12
        let result = (1..=4u64).fold(1, |acc, x| lcm(acc, x));
        assert_eq!(result, 12);
    }

    #[test]
    fn test_solve_produces_output() {
        let result = solve();
        assert!(result > 0);
        // LCM of 1..20 should be quite large
        assert!(result > 1000);
    }
}

pe_utils::pe_main!();
