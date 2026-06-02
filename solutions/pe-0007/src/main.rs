// 10001st Prime
// https://projecteuler.net/problem=7

use pe_lib::sieve_bools;

fn solve() -> usize {
    const UPPER_LIMIT: usize = 114_319;
    let primes = sieve_bools(UPPER_LIMIT);
    find_nth_prime(&primes, 10001).expect("not enough primes")
}

pe_utils::pe_main!();

fn find_nth_prime(primes: &[bool], n: usize) -> Result<usize, String> {
    let mut count = 0;

    for (i, &is_prime) in primes.iter().enumerate() {
        if is_prime {
            count += 1;

            if count == n {
                return Ok(i);
            }
        }
    }

    Err("Not enough primes found.".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_nth_prime_basic() {
        let primes = sieve_bools(30);
        assert_eq!(find_nth_prime(&primes, 1).unwrap(), 2);
        assert_eq!(find_nth_prime(&primes, 2).unwrap(), 3);
        assert_eq!(find_nth_prime(&primes, 3).unwrap(), 5);
        assert_eq!(find_nth_prime(&primes, 4).unwrap(), 7);
    }

    #[test]
    fn test_find_nth_prime_larger() {
        let primes = sieve_bools(100);
        assert_eq!(find_nth_prime(&primes, 10).unwrap(), 29);
        assert_eq!(find_nth_prime(&primes, 25).unwrap(), 97);
    }

    #[test]
    fn test_find_nth_prime_boundary() {
        let primes = sieve_bools(30);
        let last_prime_index = find_nth_prime(&primes, 10).unwrap();
        assert_eq!(last_prime_index, 29);
    }

    #[test]
    fn test_find_nth_prime_insufficient() {
        let primes = sieve_bools(10);
        assert!(find_nth_prime(&primes, 20).is_err());
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > 0);
        assert!(result < 114_319);
    }
}

