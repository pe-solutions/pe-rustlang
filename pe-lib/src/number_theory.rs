use crate::divisors::prime_factors;
use crate::sieve::sieve_bools;

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub fn totient(n: u64) -> u64 {
    if n == 1 {
        return 1;
    }
    let factors = prime_factors(n);
    let mut result = n;
    for (p, _) in factors {
        result = result / p * (p - 1);
    }
    result
}

pub fn totient_sieve(limit: usize) -> Vec<usize> {
    let mut phi = (0..=limit).collect::<Vec<_>>();
    let is_prime = sieve_bools(limit);

    for i in 2..=limit {
        if is_prime[i] {
            for j in (i..=limit).step_by(i) {
                phi[j] -= phi[j] / (i as usize);
            }
        }
    }
    phi
}

#[cfg(test)]
mod tests {
    use super::*;

    // gcd tests
    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(15, 10), 5);
        assert_eq!(gcd(21, 14), 7);
    }

    #[test]
    fn test_gcd_commutative() {
        assert_eq!(gcd(12, 8), gcd(8, 12));
        assert_eq!(gcd(100, 50), gcd(50, 100));
    }

    #[test]
    fn test_gcd_identity() {
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(7, 7), 7);
    }

    #[test]
    fn test_gcd_coprime() {
        assert_eq!(gcd(7, 11), 1);
        assert_eq!(gcd(13, 17), 1);
        assert_eq!(gcd(9, 16), 1);
    }

    #[test]
    fn test_gcd_divisibility() {
        // If a divides b, then gcd(a, b) = a
        assert_eq!(gcd(5, 15), 5);
        assert_eq!(gcd(7, 49), 7);
        assert_eq!(gcd(12, 36), 12);
    }

    // totient tests
    #[test]
    fn test_totient_small_values() {
        assert_eq!(totient(1), 1);
        assert_eq!(totient(2), 1);
        assert_eq!(totient(3), 2);
        assert_eq!(totient(4), 2);
        assert_eq!(totient(5), 4);
        assert_eq!(totient(6), 2);
    }

    #[test]
    fn test_totient_prime() {
        // For prime p, φ(p) = p - 1
        assert_eq!(totient(2), 1);
        assert_eq!(totient(3), 2);
        assert_eq!(totient(5), 4);
        assert_eq!(totient(7), 6);
        assert_eq!(totient(11), 10);
        assert_eq!(totient(13), 12);
    }

    #[test]
    fn test_totient_prime_power() {
        // For p^k, φ(p^k) = p^(k-1) * (p - 1)
        assert_eq!(totient(4), 2);    // 2^2: 2^1 * (2-1) = 2
        assert_eq!(totient(8), 4);    // 2^3: 2^2 * (2-1) = 4
        assert_eq!(totient(9), 6);    // 3^2: 3^1 * (3-1) = 6
        assert_eq!(totient(25), 20);  // 5^2: 5^1 * (5-1) = 20
    }

    #[test]
    fn test_totient_composite() {
        assert_eq!(totient(6), 2);    // 2*3
        assert_eq!(totient(10), 4);   // 2*5
        assert_eq!(totient(12), 4);   // 2^2*3
        assert_eq!(totient(30), 8);   // 2*3*5
    }

    // totient_sieve tests
    #[test]
    fn test_totient_sieve_small() {
        let phi = totient_sieve(10);
        assert_eq!(phi[1], 1);
        assert_eq!(phi[2], 1);
        assert_eq!(phi[3], 2);
        assert_eq!(phi[4], 2);
        assert_eq!(phi[5], 4);
        assert_eq!(phi[6], 2);
        assert_eq!(phi[10], 4);
    }

    #[test]
    fn test_totient_sieve_consistency() {
        let sieve = totient_sieve(30);
        // Verify that sieve values match individual totient() calls
        for n in 1..=30 {
            assert_eq!(sieve[n], totient(n as u64) as usize, "mismatch at {}", n);
        }
    }

    #[test]
    fn test_totient_sieve_length() {
        let phi = totient_sieve(100);
        assert_eq!(phi.len(), 101);
        assert_eq!(phi[0], 0);
        assert_eq!(phi[1], 1);
    }

    #[test]
    fn test_totient_sieve_primes() {
        let phi = totient_sieve(20);
        // For primes, φ(p) = p - 1
        assert_eq!(phi[2], 1);
        assert_eq!(phi[3], 2);
        assert_eq!(phi[5], 4);
        assert_eq!(phi[7], 6);
        assert_eq!(phi[11], 10);
        assert_eq!(phi[13], 12);
        assert_eq!(phi[17], 16);
        assert_eq!(phi[19], 18);
    }
}
