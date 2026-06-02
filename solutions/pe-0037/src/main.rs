// Truncatable Primes
// https://projecteuler.net/problem=37

use pe_lib::is_prime;


fn is_truncatable(t: usize) -> bool {
    let num_str = t.to_string();

    (0..(num_str.len() - 1))
        .all(|i| {
            let left = &num_str[..=i];
            let right = &num_str[i + 1..];
            is_prime(left.parse::<u64>().unwrap()) && is_prime(right.parse::<u64>().unwrap())
        })
}

fn solve() -> usize {
    (10..)
        .filter(|&i| is_prime(i as u64) && is_truncatable(i))
        .take(11)
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_truncatable_basic() {
        // 3797 is a known truncatable prime
        assert!(is_truncatable(3797));
    }

    #[test]
    fn test_is_truncatable_two_digit() {
        // 37: left=3 (prime), right=7 (prime) - both prime
        assert!(is_truncatable(37));
        // 13: left=1 (not prime), right=3 (prime) - 1 is not prime
        assert!(!is_truncatable(13));
    }

    #[test]
    fn test_is_truncatable_property() {
        // All truncatable primes must be formed from prime digits
        // Examples: 23 -> 2,3 both prime
        assert!(is_truncatable(23));
    }

    #[test]
    fn test_solve_produces_sum() {
        let result = solve();
        assert!(result > 0);
        // Sum should include 11 truncatable primes
        assert!(result > 100);
    }
}

pe_utils::pe_main!();
