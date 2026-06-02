pub fn isqrt(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

pub fn is_perfect_square(n: u64) -> bool {
    let root = isqrt(n);
    root * root == n
}

#[cfg(test)]
mod tests {
    use super::*;

    // isqrt tests
    #[test]
    fn test_isqrt_edge_cases() {
        assert_eq!(isqrt(0), 0);
        assert_eq!(isqrt(1), 1);
    }

    #[test]
    fn test_isqrt_perfect_squares() {
        assert_eq!(isqrt(4), 2);
        assert_eq!(isqrt(9), 3);
        assert_eq!(isqrt(16), 4);
        assert_eq!(isqrt(25), 5);
        assert_eq!(isqrt(100), 10);
        assert_eq!(isqrt(144), 12);
        assert_eq!(isqrt(10000), 100);
    }

    #[test]
    fn test_isqrt_non_perfect_squares() {
        assert_eq!(isqrt(2), 1);
        assert_eq!(isqrt(3), 1);
        assert_eq!(isqrt(5), 2);
        assert_eq!(isqrt(8), 2);
        assert_eq!(isqrt(10), 3);
        assert_eq!(isqrt(15), 3);
        assert_eq!(isqrt(24), 4);
        assert_eq!(isqrt(26), 5);
    }

    #[test]
    fn test_isqrt_floor_property() {
        // isqrt(n) should be floor(sqrt(n))
        for n in [2, 3, 5, 10, 99, 100, 101, 1000, 10000, 100000] {
            let root = isqrt(n);
            assert!(root * root <= n);
            if root + 1 <= u64::MAX / (root + 1) {
                assert!((root + 1) * (root + 1) > n);
            }
        }
    }

    #[test]
    fn test_isqrt_large_values() {
        // Large values below MAX to avoid overflow in Newton-Raphson
        assert_eq!(isqrt(1000000000000), 1000000);
        assert_eq!(isqrt(999999999999), 999999);
        assert_eq!(isqrt(9223372036854775807), 3037000499); // close to i64::MAX
    }

    // is_perfect_square tests
    #[test]
    fn test_is_perfect_square_true() {
        assert!(is_perfect_square(0));
        assert!(is_perfect_square(1));
        assert!(is_perfect_square(4));
        assert!(is_perfect_square(9));
        assert!(is_perfect_square(16));
        assert!(is_perfect_square(25));
        assert!(is_perfect_square(100));
        assert!(is_perfect_square(10000));
    }

    #[test]
    fn test_is_perfect_square_false() {
        assert!(!is_perfect_square(2));
        assert!(!is_perfect_square(3));
        assert!(!is_perfect_square(5));
        assert!(!is_perfect_square(8));
        assert!(!is_perfect_square(10));
        assert!(!is_perfect_square(99));
        assert!(!is_perfect_square(101));
        assert!(!is_perfect_square(1000));
    }

    #[test]
    fn test_is_perfect_square_large() {
        assert!(is_perfect_square(1000000));
        assert!(!is_perfect_square(999999));
        assert!(!is_perfect_square(1000001));
        assert!(is_perfect_square(1000000000000));
    }
}
