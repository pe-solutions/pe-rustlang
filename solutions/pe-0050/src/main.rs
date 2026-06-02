// Consecutive prime sum
// https://projecteuler.net/problem=50

use std::vec::Vec;
use pe_lib::is_prime;

fn is_prime_i64(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    is_prime(n as u64)
}

fn solve() -> i64 {
    const LIMIT: i64 = 1_000_000;
    let primes: Vec<i64> = (2..LIMIT).filter(|&x| is_prime_i64(x)).collect();
    let mut max_sum = 0;
    let mut max_length = 0;
    for i in 0..primes.len() {
        let mut sum = primes[i];
        for j in i + 1..primes.len() {
            sum += primes[j];
            if is_prime_i64(sum) {
                let length = j - i + 1;
                if length > max_length {
                    max_length = length;
                    max_sum = sum;
                }
            }
            if sum >= LIMIT { break; }
        }
    }
    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime_i64_basic() {
        assert!(is_prime_i64(2));
        assert!(is_prime_i64(3));
        assert!(is_prime_i64(5));
        assert!(!is_prime_i64(4));
        assert!(!is_prime_i64(1));
        assert!(!is_prime_i64(0));
    }

    #[test]
    fn test_prime_sum_property() {
        // Example: 2+3 = 5 (prime), 2+3+5 = 10 (not prime)
        assert!(is_prime_i64(5));
        assert!(!is_prime_i64(10));
    }

    #[test]
    fn test_consecutive_sum() {
        // Small test: sum of first few primes
        let sum = 2i64 + 3 + 5; // = 10, not prime
        assert!(!is_prime_i64(sum));
        let sum2 = 2i64 + 3 + 5 + 7; // = 17, prime
        assert!(is_prime_i64(sum2));
    }

    #[test]
    fn test_solve_produces_prime() {
        let result = solve();
        assert!(is_prime_i64(result));
        assert!(result > 100);
    }
}

pe_utils::pe_main!();
