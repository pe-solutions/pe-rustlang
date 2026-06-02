pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next_value = self.a;
        self.a = self.b;
        self.b = next_value + self.b;
        Some(next_value)
    }
}

pub fn triangular(n: u64) -> u64 {
    n * (n + 1) / 2
}

pub fn pentagonal(n: u64) -> u64 {
    n * (3 * n - 1) / 2
}

pub fn hexagonal(n: u64) -> u64 {
    n * (2 * n - 1)
}

pub fn heptagonal(n: u64) -> u64 {
    n * (5 * n - 3) / 2
}

pub fn octagonal(n: u64) -> u64 {
    n * (3 * n - 2)
}

pub fn is_triangular(n: u64) -> bool {
    let discriminant = 1 + 8 * n;
    let sqrt_disc = (discriminant as f64).sqrt() as u64;
    if sqrt_disc * sqrt_disc != discriminant {
        return false;
    }
    (sqrt_disc + 1) % 2 == 0 && (sqrt_disc - 1) / 2 > 0
}

pub fn is_pentagonal(n: u64) -> bool {
    let num = 24 * n + 1;
    let sqrt_num = (num as f64).sqrt() as u64;
    sqrt_num * sqrt_num == num && (sqrt_num + 1) % 6 == 0
}

pub fn is_hexagonal(n: u64) -> bool {
    let num = 8 * n + 1;
    let sqrt_num = (num as f64).sqrt() as u64;
    sqrt_num * sqrt_num == num && (sqrt_num + 1) % 4 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fibonacci tests
    #[test]
    fn test_fibonacci_first_10() {
        let fib: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_fibonacci_properties() {
        let mut fib = Fibonacci::new();
        let f0 = fib.next().unwrap(); // 0
        let f1 = fib.next().unwrap(); // 1
        let f2 = fib.next().unwrap(); // 1
        let f3 = fib.next().unwrap(); // 2
        let f4 = fib.next().unwrap(); // 3
        let f5 = fib.next().unwrap(); // 5
        assert_eq!(f2, f0 + f1);
        assert_eq!(f3, f1 + f2);
        assert_eq!(f4, f2 + f3);
        assert_eq!(f5, f3 + f4);
    }

    #[test]
    fn test_fibonacci_default() {
        let fib: Vec<u64> = Fibonacci::default().take(5).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3]);
    }

    #[test]
    fn test_fibonacci_large() {
        let fib: Vec<u64> = Fibonacci::new().take(90).collect();
        assert_eq!(fib.len(), 90);
        assert_eq!(fib[20], 6765);
        // fib[30] = 832040, but by fib[93] we start overflowing u64
    }

    // Triangular number tests
    #[test]
    fn test_triangular_formula() {
        assert_eq!(triangular(1), 1);
        assert_eq!(triangular(2), 3);
        assert_eq!(triangular(3), 6);
        assert_eq!(triangular(4), 10);
        assert_eq!(triangular(5), 15);
        assert_eq!(triangular(10), 55);
    }

    #[test]
    fn test_triangular_known_values() {
        let triangulars: Vec<u64> = (1..=10).map(triangular).collect();
        assert_eq!(triangulars, vec![1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
    }

    // Pentagonal number tests
    #[test]
    fn test_pentagonal_formula() {
        assert_eq!(pentagonal(1), 1);
        assert_eq!(pentagonal(2), 5);
        assert_eq!(pentagonal(3), 12);
        assert_eq!(pentagonal(4), 22);
        assert_eq!(pentagonal(5), 35);
    }

    #[test]
    fn test_pentagonal_growth() {
        for n in 1..10 {
            assert!(pentagonal(n) < pentagonal(n + 1));
        }
    }

    // Hexagonal number tests
    #[test]
    fn test_hexagonal_formula() {
        assert_eq!(hexagonal(1), 1);
        assert_eq!(hexagonal(2), 6);
        assert_eq!(hexagonal(3), 15);
        assert_eq!(hexagonal(4), 28);
        assert_eq!(hexagonal(5), 45);
    }

    // Heptagonal number tests
    #[test]
    fn test_heptagonal_formula() {
        assert_eq!(heptagonal(1), 1);
        assert_eq!(heptagonal(2), 7);
        assert_eq!(heptagonal(3), 18);
        assert_eq!(heptagonal(4), 34);
        assert_eq!(heptagonal(5), 55);
    }

    // Octagonal number tests
    #[test]
    fn test_octagonal_formula() {
        assert_eq!(octagonal(1), 1);
        assert_eq!(octagonal(2), 8);
        assert_eq!(octagonal(3), 21);
        assert_eq!(octagonal(4), 40);
        assert_eq!(octagonal(5), 65);
    }

    // is_triangular tests
    #[test]
    fn test_is_triangular_true() {
        assert!(is_triangular(1));
        assert!(is_triangular(3));
        assert!(is_triangular(6));
        assert!(is_triangular(10));
        assert!(is_triangular(55));
    }

    #[test]
    fn test_is_triangular_false() {
        assert!(!is_triangular(2));
        assert!(!is_triangular(4));
        assert!(!is_triangular(5));
        assert!(!is_triangular(7));
    }

    // is_pentagonal tests
    #[test]
    fn test_is_pentagonal_true() {
        assert!(is_pentagonal(1));
        assert!(is_pentagonal(5));
        assert!(is_pentagonal(12));
        assert!(is_pentagonal(22));
        assert!(is_pentagonal(35));
    }

    #[test]
    fn test_is_pentagonal_false() {
        assert!(!is_pentagonal(2));
        assert!(!is_pentagonal(6));
        assert!(!is_pentagonal(10));
        assert!(!is_pentagonal(15));
    }

    // is_hexagonal tests
    #[test]
    fn test_is_hexagonal_true() {
        assert!(is_hexagonal(1));
        assert!(is_hexagonal(6));
        assert!(is_hexagonal(15));
        assert!(is_hexagonal(28));
        assert!(is_hexagonal(45));
    }

    #[test]
    fn test_is_hexagonal_false() {
        assert!(!is_hexagonal(2));
        assert!(!is_hexagonal(5));
        assert!(!is_hexagonal(10));
        assert!(!is_hexagonal(20));
    }
}
