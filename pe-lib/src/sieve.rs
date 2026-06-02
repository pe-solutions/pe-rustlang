pub fn sieve_bools(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit + 1];
    if limit > 0 {
        is_prime[0] = false;
    }
    if limit > 1 {
        is_prime[1] = false;
    }
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
}

pub fn sieve_primes(limit: usize) -> Vec<usize> {
    let is_prime = sieve_bools(limit);
    (2..=limit).filter(|&i| is_prime[i]).collect()
}

pub fn sieve_omega(limit: usize) -> Vec<usize> {
    let mut omega = vec![0; limit];
    for i in 2..limit {
        if omega[i] == 0 {
            for j in (i..limit).step_by(i) {
                omega[j] += 1;
            }
        }
    }
    omega
}

#[cfg(test)]
mod tests {
    use super::*;

    // sieve_bools tests
    #[test]
    fn test_sieve_bools_edge_cases() {
        // Edge cases: degenerate but well-defined
        let result0 = sieve_bools(0);
        assert_eq!(result0.len(), 1);
        let result1 = sieve_bools(1);
        assert_eq!(result1.len(), 2);
        assert_eq!(result1[0], false); // 0 is not prime
        assert_eq!(result1[1], true);  // 1 is marked as prime (degenerate case, guard doesn't apply)
    }

    #[test]
    fn test_sieve_bools_small() {
        let sieve = sieve_bools(10);
        assert_eq!(sieve[0], false); // 0 is not prime
        assert_eq!(sieve[1], false); // 1 is not prime
        assert_eq!(sieve[2], true);  // 2 is prime
        assert_eq!(sieve[3], true);  // 3 is prime
        assert_eq!(sieve[4], false); // 4 is not prime
        assert_eq!(sieve[5], true);  // 5 is prime
        assert_eq!(sieve[10], false); // 10 is not prime
    }

    #[test]
    fn test_sieve_bools_correct_count() {
        // First 25 primes below 100
        let sieve = sieve_bools(100);
        let prime_count: usize = sieve.iter().filter(|&&p| p).count();
        assert_eq!(prime_count, 25); // There are 25 primes ≤ 100
    }

    #[test]
    fn test_sieve_bools_known_composite() {
        let sieve = sieve_bools(20);
        for &n in &[4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20] {
            assert_eq!(sieve[n], false, "{} should not be prime", n);
        }
    }

    // sieve_primes tests
    #[test]
    fn test_sieve_primes_small() {
        assert_eq!(sieve_primes(1), vec![]);
        assert_eq!(sieve_primes(2), vec![2]);
        assert_eq!(sieve_primes(10), vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_sieve_primes_first_20() {
        let primes = sieve_primes(71);
        // First 20 primes: 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71];
        assert_eq!(primes, expected);
    }

    #[test]
    fn test_sieve_primes_length() {
        let primes = sieve_primes(100);
        assert_eq!(primes.len(), 25); // 25 primes below 100
    }

    #[test]
    fn test_sieve_primes_all_prime() {
        let primes = sieve_primes(50);
        // Check that all returned numbers are actually prime by trial division
        for &p in &primes {
            let is_prime = if p < 2 {
                false
            } else {
                !(2..p).any(|i| p % i == 0)
            };
            assert!(is_prime, "{} should be prime", p);
        }
    }

    // sieve_omega tests
    #[test]
    fn test_sieve_omega_small() {
        let omega = sieve_omega(10);
        assert_eq!(omega[0], 0); // By convention, 0 has 0 prime factors
        assert_eq!(omega[1], 0); // 1 has 0 distinct prime factors
        assert_eq!(omega[2], 1); // 2 = 2^1
        assert_eq!(omega[3], 1); // 3 = 3^1
        assert_eq!(omega[4], 1); // 4 = 2^2 (one distinct prime)
        assert_eq!(omega[5], 1); // 5 = 5^1
        assert_eq!(omega[6], 2); // 6 = 2*3 (two distinct primes)
    }

    #[test]
    fn test_sieve_omega_distinct_prime_counts() {
        let omega = sieve_omega(31);  // Need limit > 30 to access index 30
        assert_eq!(omega[2], 1);   // 2
        assert_eq!(omega[4], 1);   // 2^2
        assert_eq!(omega[6], 2);   // 2*3
        assert_eq!(omega[8], 1);   // 2^3
        assert_eq!(omega[12], 2);  // 2^2*3
        assert_eq!(omega[15], 2);  // 3*5
        assert_eq!(omega[30], 3);  // 2*3*5
    }

    #[test]
    fn test_sieve_omega_primes_are_one() {
        let omega = sieve_omega(50);
        let expected_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        for &p in &expected_primes {
            assert_eq!(omega[p], 1, "prime {} should have omega=1", p);
        }
    }

    #[test]
    fn test_sieve_omega_composite_counts() {
        let omega = sieve_omega(20);
        // Composites with 2 distinct prime factors: 6, 10, 14, 15
        assert_eq!(omega[6], 2);
        assert_eq!(omega[10], 2);
        assert_eq!(omega[14], 2);
        assert_eq!(omega[15], 2);
        // Composites with 1 distinct prime factor: 4, 8, 9, 16
        assert_eq!(omega[4], 1);
        assert_eq!(omega[8], 1);
        assert_eq!(omega[9], 1);
        assert_eq!(omega[16], 1);
    }
}
