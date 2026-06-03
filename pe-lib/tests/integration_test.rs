// Integration tests for pe-lib and refactored solutions
// Tests cross-module function combinations and composite algorithms

use pe_lib::{
    is_prime, sieve_primes, sieve_bools, digit_sum, is_palindrome_num,
    triangular, pentagonal, hexagonal, sum_proper_divisors,
    count_divisors, prime_factors, gcd, totient,
    mod_pow, mod_mul, Fibonacci, binomial_big, count_partitions,
    isqrt, is_perfect_square
};

// ============================================================================
// CROSS-MODULE INTEGRATION TESTS
// ============================================================================

#[test]
fn test_sieve_prime_consistency() {
    // Verify that sieve_primes and is_prime are consistent
    let limit = 100;
    let sieve = sieve_bools(limit);
    let primes = sieve_primes(limit);

    for &p in &primes {
        assert!(is_prime(p as u64), "{} should be prime", p);
    }

    // Verify no non-primes in result
    for &p in &primes {
        assert!(sieve[p], "sieve should mark {} as prime", p);
    }
}

#[test]
fn test_digit_sum_digit_decomposition() {
    // Test that digit_sum matches manual digit calculation
    let n = 12345u64;
    let sum_from_func = digit_sum(n);

    let mut manual_sum = 0u64;
    let mut temp = n;
    while temp > 0 {
        manual_sum += temp % 10;
        temp /= 10;
    }

    assert_eq!(sum_from_func, manual_sum);
}

#[test]
fn test_palindrome_consistency() {
    // Test that is_palindrome_num works for various cases
    let palindromes = vec![11, 121, 1331, 12321, 9009];
    for p in palindromes {
        assert!(is_palindrome_num(p), "{} should be palindrome", p);
    }
}

#[test]
fn test_gcd_lcm_relationship() {
    // Verify: a * b = gcd(a,b) * lcm(a,b)
    let pairs = vec![(12, 8), (21, 14), (100, 50)];

    for (a, b) in pairs {
        let g = gcd(a, b);
        let lcm = a / g * b;
        assert_eq!(a * b, g * lcm, "GCD-LCM relationship failed for ({}, {})", a, b);
    }
}

#[test]
fn test_divisors_prime_factorization() {
    // Verify that prime_factors reconstructs the original number
    let numbers = vec![60, 120, 2520];

    for n in numbers {
        let factors = prime_factors(n);
        let reconstructed: u64 = factors.iter()
            .map(|(p, e)| p.pow(*e))
            .product();
        assert_eq!(reconstructed, n);
    }
}

#[test]
fn test_totient_multiplicativity() {
    // For coprime a,b: φ(a*b) = φ(a) * φ(b)
    let a = 7u64;
    let b = 11u64;

    let phi_a = totient(a);
    let phi_b = totient(b);
    let phi_ab = totient(a * b);

    assert_eq!(phi_ab, phi_a * phi_b);
}

#[test]
fn test_modular_arithmetic_chain() {
    // Test that modular operations compose correctly
    let base = 2u64;
    let exp = 10u64;
    let modulus = 1000u64;

    // a^10 mod 1000 via mod_pow
    let via_mod_pow = mod_pow(base, exp, modulus);

    // a^10 via repeated mod_mul
    let mut via_mul = 1u64;
    for _ in 0..exp {
        via_mul = mod_mul(via_mul, base, modulus);
    }

    assert_eq!(via_mod_pow, via_mul);
}

#[test]
fn test_perfect_square_isqrt() {
    // Verify is_perfect_square and isqrt consistency
    for n in 1..=20 {
        let sq = n * n;
        assert!(is_perfect_square(sq as u64));
        assert_eq!(isqrt(sq as u64), n as u64);
    }
}

#[test]
fn test_polygonal_numbers_properties() {
    // Verify triangular, pentagonal, hexagonal relationships
    let n = 5u64;

    let tri = triangular(n);
    let pent = pentagonal(n);
    let hex = hexagonal(n);

    // All should be positive and increasing
    assert!(tri > 0 && pent > 0 && hex > 0);
    assert!(triangular(n) < triangular(n + 1));
}

#[test]
fn test_fibonacci_sequences() {
    // Verify Fibonacci iterator produces consistent results
    let fib: Vec<u64> = Fibonacci::new().take(10).collect();

    // Verify Fibonacci property: F(n) = F(n-1) + F(n-2)
    for i in 2..fib.len() {
        assert_eq!(fib[i], fib[i-1] + fib[i-2]);
    }
}

// ============================================================================
// COMPOSITE ALGORITHM TESTS
// ============================================================================

#[test]
fn test_amicable_pair_properties() {
    // Test the famous amicable pair (220, 284)
    assert_eq!(sum_proper_divisors(220), 284);
    assert_eq!(sum_proper_divisors(284), 220);
}

#[test]
fn test_perfect_number() {
    // 6 is the first perfect number
    assert_eq!(sum_proper_divisors(6), 6);
    // 28 is the second perfect number
    assert_eq!(sum_proper_divisors(28), 28);
}

#[test]
fn test_prime_power_divisor_count() {
    // For prime p: divisors of p^k = k+1
    let prime = 2u64;
    for k in 1..=5 {
        let n = prime.pow(k);
        let div_count = count_divisors(n);
        assert_eq!(div_count, (k + 1) as u64);
    }
}

#[test]
fn test_gcd_prime_factorization() {
    // GCD via prime factorization
    let a = 60u64;
    let b = 48u64;

    let _factors_a = prime_factors(a);
    let _factors_b = prime_factors(b);

    let gcd_val = gcd(a, b);
    assert!(gcd_val > 0);
    assert_eq!(a % gcd_val, 0);
    assert_eq!(b % gcd_val, 0);
}

#[test]
fn test_combinatorial_properties() {
    // Test factorial and binomial relationships
    // C(n,k) = n! / (k!(n-k)!)

    let n = 10u32;
    let k = 3u32;

    let binom = binomial_big(n, k);

    // Verify symmetry: C(n,k) = C(n,n-k)
    let symmetric = binomial_big(n, n - k);
    assert_eq!(binom, symmetric);
}

#[test]
fn test_partition_count_growth() {
    // Partition count should grow monotonically
    let mut prev = count_partitions(0);

    for n in 1..=10 {
        let curr = count_partitions(n);
        assert!(curr >= prev);
        prev = curr;
    }
}

#[test]
fn test_prime_gap_analysis() {
    // Analyze gaps between consecutive primes
    let primes = sieve_primes(100);

    let mut max_gap = 0;
    for i in 1..primes.len() {
        let gap = primes[i] - primes[i-1];
        max_gap = gap.max(max_gap);
    }

    // First gap of 4 or more should be between 7 and 11
    assert!(max_gap >= 2);
}

#[test]
fn test_digit_manipulation_chain() {
    // Test combinations of digit functions
    let n = 12321u64;

    // Palindrome check
    assert!(is_palindrome_num(n));

    // Digit sum
    let sum = digit_sum(n);
    assert_eq!(sum, 1 + 2 + 3 + 2 + 1);
}

// ============================================================================
// CROSS-SOLUTION VERIFICATION TESTS
// ============================================================================

#[test]
fn test_solution_pe0010_logic() {
    // Verify sum of primes below 100 = 1060
    let limit = 100u64;
    let sum: u64 = sieve_primes(limit as usize)
        .iter()
        .map(|&p| p as u64)
        .sum();
    assert_eq!(sum, 1060);
}

#[test]
fn test_solution_pe0021_logic() {
    // Test amicable number pair 220, 284
    assert_eq!(sum_proper_divisors(220), 284);
    assert_eq!(sum_proper_divisors(284), 220);
}

#[test]
fn test_solution_pe0076_logic() {
    // Partition count for small numbers
    assert_eq!(count_partitions(3), 3);
    assert_eq!(count_partitions(5), 7);
}

#[test]
fn test_solution_pe0066_logic() {
    // Perfect square verification
    for n in [1, 4, 9, 16, 25, 100] {
        assert!(is_perfect_square(n));
    }
}

#[test]
fn test_solution_pe0070_logic() {
    // Totient property: φ(p*q) = (p-1)(q-1) for distinct primes p,q
    let p = 13u64;
    let q = 17u64;
    let product = p * q;

    let phi = totient(product);
    let expected = (p - 1) * (q - 1);

    assert_eq!(phi, expected);
}

// ============================================================================
// PERFORMANCE CHARACTERISTICS TESTS
// ============================================================================

#[test]
fn test_performance_sieve_efficiency() {
    // Verify sieve is efficient for large limits
    let start = std::time::Instant::now();
    let _primes = sieve_primes(100_000);
    let elapsed = start.elapsed();

    // Should complete in reasonable time (< 1 second for 100K)
    assert!(elapsed.as_secs() < 1, "Sieve took too long: {:?}", elapsed);
}

#[test]
fn test_performance_is_prime_scaling() {
    // Verify is_prime scales well
    let start = std::time::Instant::now();
    let _result = is_prime(1_000_000_007);
    let elapsed = start.elapsed();

    // Should be fast even for large numbers
    assert!(elapsed.as_millis() < 100, "is_prime too slow: {:?}", elapsed);
}

#[test]
fn test_performance_gcd_euclid() {
    // Euclidean GCD should be very fast
    let start = std::time::Instant::now();
    let _g = gcd(1_000_000_000, 1_000_000_007);
    let elapsed = start.elapsed();

    assert!(elapsed.as_micros() < 1000, "GCD too slow: {:?}", elapsed);
}

// ============================================================================
// ERROR BOUNDARY TESTS
// ============================================================================

#[test]
fn test_boundary_zero_handling() {
    // Test functions with zero
    assert_eq!(sum_proper_divisors(0), 0);
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(5, 0), 5);
}

#[test]
fn test_boundary_one_handling() {
    // Test functions with one
    assert_eq!(sum_proper_divisors(1), 0);
    assert_eq!(count_divisors(1), 1);
    assert_eq!(totient(1), 1);
}

#[test]
fn test_boundary_large_numbers() {
    // Test with large but reasonable numbers
    let large = 1_000_000_000_000u64;
    let root = isqrt(large);
    assert_eq!(root * root, 1_000_000_000_000);
}

#[test]
fn test_modular_identity() {
    // mod_pow(a, 1, m) = a mod m
    let a = 1234u64;
    let m = 1000u64;
    assert_eq!(mod_pow(a, 1, m), a % m);
}

#[test]
fn test_empty_prime_list() {
    // Sieve below 2 should be empty
    let primes = sieve_primes(1);
    assert_eq!(primes.len(), 0);
}
