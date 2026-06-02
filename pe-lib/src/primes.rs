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
    for &a in &[2u64, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        if !miller_rabin_test(a, d, shift, n) {
            return false;
        }
    }
    true
}

fn decompose(mut n: u64) -> (u64, u32) {
    let mut shift = 0;
    while n % 2 == 0 {
        n /= 2;
        shift += 1;
    }
    (n, shift)
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
