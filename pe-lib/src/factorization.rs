pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in &[2, 3, 5, 7, 11, 13] {
        if n % p == 0 {
            return n == p;
        }
    }

    let (d, shift) = decompose(n - 1);
    for &a in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if a >= n {
            continue;
        }
        if !miller_rabin_test(a, d, shift, n) {
            return false;
        }
    }
    true
}

fn decompose(n: u64) -> (u64, u32) {
    let shift = n.trailing_zeros();
    (n >> shift, shift)
}

fn miller_rabin_test(a: u64, d: u64, shift: u32, n: u64) -> bool {
    let mut x = mod_pow_for_miller_rabin(a as u128, d as u128, n as u128) as u64;
    if x == 1 || x == n - 1 {
        return true;
    }
    for _ in 1..shift {
        x = mod_mul_for_miller_rabin(x as u128, x as u128, n as u128) as u64;
        if x == n - 1 {
            return true;
        }
    }
    false
}

fn mod_pow_for_miller_rabin(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }
    result
}

fn mod_mul_for_miller_rabin(a: u128, b: u128, modulus: u128) -> u128 {
    ((a % modulus) * (b % modulus)) % modulus
}

pub fn is_prime_trial(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn prime_factors(mut n: u64) -> Vec<(u64, u32)> {
    let mut factors = Vec::new();
    let mut count = 0;
    while n % 2 == 0 {
        n /= 2;
        count += 1;
    }
    if count > 0 {
        factors.push((2, count));
    }
    count = 0;
    while n % 3 == 0 {
        n /= 3;
        count += 1;
    }
    if count > 0 {
        factors.push((3, count));
    }
    let mut divisor = 5;
    while divisor * divisor <= n {
        count = 0;
        while n % divisor == 0 {
            n /= divisor;
            count += 1;
        }
        if count > 0 {
            factors.push((divisor, count));
        }
        count = 0;
        let next = divisor + 2;
        while n % next == 0 {
            n /= next;
            count += 1;
        }
        if count > 0 {
            factors.push((next, count));
        }
        divisor += 6;
    }
    if n > 1 {
        factors.push((n, 1));
    }
    factors
}

pub fn largest_prime_factor(mut n: u64) -> u64 {
    let mut largest = 1u64;
    while n % 2 == 0 {
        largest = 2;
        n /= 2;
    }
    while n % 3 == 0 {
        largest = 3;
        n /= 3;
    }
    let mut i: u64 = 5;
    while i * i <= n {
        while n % i == 0 {
            largest = i;
            n /= i;
        }
        let next = i + 2;
        while n % next == 0 {
            largest = next;
            n /= next;
        }
        i += 6;
    }
    if n > 1 {
        largest = n;
    }
    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    // is_prime tests
    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(13));
        assert!(is_prime(17));
        assert!(is_prime(19));
        assert!(is_prime(23));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(9));
        assert!(is_prime(97));
        assert!(!is_prime(100));
    }

    #[test]
    fn test_is_prime_trial() {
        assert!(!is_prime_trial(0));
        assert!(!is_prime_trial(1));
        assert!(is_prime_trial(2));
        assert!(is_prime_trial(3));
        assert!(is_prime_trial(5));
        assert!(is_prime_trial(7));
        assert!(!is_prime_trial(4));
        assert!(!is_prime_trial(6));
        assert!(is_prime_trial(97));
    }

    // prime_factors tests
    #[test]
    fn test_prime_factors_small() {
        assert_eq!(prime_factors(2), vec![(2, 1)]);
        assert_eq!(prime_factors(6), vec![(2, 1), (3, 1)]);
        assert_eq!(prime_factors(12), vec![(2, 2), (3, 1)]);
    }

    #[test]
    fn test_prime_factors_powers() {
        assert_eq!(prime_factors(8), vec![(2, 3)]);
        assert_eq!(prime_factors(27), vec![(3, 3)]);
        assert_eq!(prime_factors(32), vec![(2, 5)]);
    }

    #[test]
    fn test_prime_factors_composite() {
        assert_eq!(prime_factors(30), vec![(2, 1), (3, 1), (5, 1)]);
        assert_eq!(prime_factors(60), vec![(2, 2), (3, 1), (5, 1)]);
    }

    #[test]
    fn test_prime_factors_reconstruction() {
        for n in [6, 12, 30, 60, 100, 120] {
            let factors = prime_factors(n);
            let product: u64 = factors.iter().map(|(p, e)| p.pow(*e)).product();
            assert_eq!(product, n);
        }
    }

    // largest_prime_factor tests
    #[test]
    fn test_largest_prime_factor_prime() {
        assert_eq!(largest_prime_factor(7), 7);
        assert_eq!(largest_prime_factor(13), 13);
        assert_eq!(largest_prime_factor(17), 17);
    }

    #[test]
    fn test_largest_prime_factor_composite() {
        assert_eq!(largest_prime_factor(6), 3);
        assert_eq!(largest_prime_factor(12), 3);
        assert_eq!(largest_prime_factor(30), 5);
        assert_eq!(largest_prime_factor(15), 5);
    }

    #[test]
    fn test_largest_prime_factor_small_powers() {
        assert_eq!(largest_prime_factor(4), 2);
        assert_eq!(largest_prime_factor(8), 2);
        assert_eq!(largest_prime_factor(9), 3);
        assert_eq!(largest_prime_factor(25), 5);
    }
}
