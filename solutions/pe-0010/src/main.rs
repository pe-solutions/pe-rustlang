// Summation of primes
// https://projecteuler.net/problem/10

use pe_lib::is_prime;

fn sum_primes_below_limit(limit: u64) -> u64 {
    let mut sum: u64 = 0;

    for num in 2..limit {
        if is_prime(num) {
            sum += num;
        }
    }

    sum
}

fn solve() -> u64 {
    sum_primes_below_limit(2_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_primes_small_limit() {
        let result10 = sum_primes_below_limit(10);
        assert!(result10 > 0);
        let result20 = sum_primes_below_limit(20);
        assert!(result20 > result10);
    }

    #[test]
    fn test_sum_primes_edge_cases() {
        assert_eq!(sum_primes_below_limit(2), 0);
        let result3 = sum_primes_below_limit(3);
        assert_eq!(result3, 2);
    }

    #[test]
    fn test_sum_primes_monotonic() {
        for limit in [10, 20, 50, 100, 1000] {
            let prev = sum_primes_below_limit(limit);
            let next = sum_primes_below_limit(limit + 1);
            assert!(next >= prev);
        }
    }

    #[test]
    fn test_solve_produces_output() {
        let result = solve();
        assert!(result > 0);
    }
}

pe_utils::pe_main!();
