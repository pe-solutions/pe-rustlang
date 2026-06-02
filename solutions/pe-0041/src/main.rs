// Pandigital Prime
// https://projecteuler.net/problem=41

extern crate itertools;

use itertools::Itertools;
use pe_lib::is_prime_trial as is_prime;

fn solve() -> u64 {
    let digits = vec!['7', '6', '5', '4', '3', '2', '1'];
    let permutations = digits.iter().permutations(digits.len());
    permutations
        .filter_map(|perm| {
            let n: u64 = perm.into_iter().collect::<String>().parse().unwrap();
            if is_prime(n) { Some(n) } else { None }
        })
        .max()
        .expect("no prime permutation found")
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_permutation_generation() {
        let digits = vec!['1', '2', '3'];
        let perms: Vec<_> = digits.iter().permutations(3).map(|p| {
            p.into_iter().collect::<String>()
        }).collect();
        assert_eq!(perms.len(), 6); // 3! = 6
    }

    #[test]
    fn test_prime_filtering() {
        // Test that we correctly identify primes
        assert!(is_prime(2));
        assert!(is_prime(7));
        assert!(is_prime(41));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
    }

    #[test]
    fn test_solve_produces_prime() {
        let result = solve();
        assert!(is_prime(result));
        assert!(result > 1000);
    }

    #[test]
    fn test_pandigital_property() {
        // Result should be pandigital (using digits 1-7)
        let result = solve();
        let result_str = result.to_string();
        assert_eq!(result_str.len(), 7);
    }
}

pe_utils::pe_main!();
