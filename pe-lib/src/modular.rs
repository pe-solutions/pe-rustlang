pub fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    (((a as u128) * (b as u128)) % (m as u128)) as u64
}

pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = mod_mul(result, base, modulus);
        }
        base = mod_mul(base, base, modulus);
        exp /= 2;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // mod_mul tests
    #[test]
    fn test_mod_mul_basic() {
        assert_eq!(mod_mul(2, 3, 5), 1); // (2*3) % 5 = 1
        assert_eq!(mod_mul(4, 3, 7), 5); // (4*3) % 7 = 5
        assert_eq!(mod_mul(5, 5, 11), 3); // (5*5) % 11 = 3
    }

    #[test]
    fn test_mod_mul_identity() {
        assert_eq!(mod_mul(5, 1, 11), 5);
        assert_eq!(mod_mul(1, 7, 13), 7);
        assert_eq!(mod_mul(10, 0, 17), 0);
    }

    #[test]
    fn test_mod_mul_overflow_safety() {
        // Test values that would overflow u64 without u128 upcast
        let max = u64::MAX;
        let result = mod_mul(max, max, 1000007);
        assert!(result < 1000007);
    }

    #[test]
    fn test_mod_mul_commutative() {
        assert_eq!(mod_mul(6, 7, 13), mod_mul(7, 6, 13));
        assert_eq!(mod_mul(123, 456, 789), mod_mul(456, 123, 789));
    }

    #[test]
    fn test_mod_mul_associative() {
        let a = mod_mul(2, 3, 17);
        let b = mod_mul(a, 5, 17);
        let c = mod_mul(3, 5, 17);
        let d = mod_mul(2, c, 17);
        assert_eq!(b, d); // (2*3)*5 == 2*(3*5) mod 17
    }

    // mod_pow tests
    #[test]
    fn test_mod_pow_base_cases() {
        assert_eq!(mod_pow(2, 0, 7), 1); // Any number^0 = 1
        assert_eq!(mod_pow(3, 1, 7), 3); // Any number^1 = number
        assert_eq!(mod_pow(0, 5, 7), 0); // 0^n = 0
    }

    #[test]
    fn test_mod_pow_small_values() {
        assert_eq!(mod_pow(2, 3, 11), 8); // 2^3 = 8
        assert_eq!(mod_pow(3, 4, 17), 81 % 17); // 3^4 = 81
        assert_eq!(mod_pow(5, 5, 13), 3125 % 13); // 5^5 mod 13
    }

    #[test]
    fn test_mod_pow_large_exponent() {
        // Test with large exponents
        let result = mod_pow(2, 100, 1000000007);
        assert!(result < 1000000007);
        let result2 = mod_pow(10, 1000, 1000000007);
        assert!(result2 < 1000000007);
    }

    #[test]
    fn test_mod_pow_fermat_little_theorem() {
        // If p is prime and gcd(a,p)=1, then a^(p-1) ≡ 1 (mod p)
        assert_eq!(mod_pow(2, 6, 7), 1); // 2^6 mod 7
        assert_eq!(mod_pow(3, 10, 11), 1); // 3^10 mod 11
        assert_eq!(mod_pow(5, 12, 13), 1); // 5^12 mod 13
    }

    #[test]
    fn test_mod_pow_overflow_safety() {
        // Test with large base and exponent
        let result = mod_pow(u64::MAX - 1, u64::MAX - 1, 1000000007);
        assert!(result < 1000000007);
    }
}
