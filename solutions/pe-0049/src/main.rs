// Prime permutations
// https://projecteuler.net/problem=49

use pe_lib::is_prime;

fn is_permutation(n1: i32, n2: i32) -> bool {
    let mut n1_digits: Vec<char> = n1.to_string().chars().collect();
    let mut n2_digits: Vec<char> = n2.to_string().chars().collect();

    n1_digits.sort();
    n2_digits.sort();

    n1_digits == n2_digits
}

fn prime_list(s: i32, e: i32) -> Vec<i32> {
    (s..=e).filter(|&n| is_prime(n as u64)).collect()
}

fn prime_permutations(primes: &Vec<i32>) -> Vec<i32> {
    for p1 in 0..primes.len() {
        for p2 in (p1 + 1)..primes.len() {
            let d1 = primes[p2] - primes[p1];

            for p3 in (p2 + 1)..primes.len() {
                let d2 = primes[p3] - primes[p2];

                if d1 == d2 && is_permutation(primes[p1], primes[p2]) && is_permutation(primes[p2], primes[p3]) {
                    return vec![primes[p1], primes[p2], primes[p3]];
                }
            }
        }
    }

    vec![]
}

fn solve() -> String {
    let primes = prime_list(1_488, 10_000);
    let result = prime_permutations(&primes);
    result.iter().map(|&n| n.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation_basic() {
        assert!(is_permutation(123, 321));
        assert!(is_permutation(1234, 4321));
        assert!(!is_permutation(123, 124));
        assert!(!is_permutation(12, 123));
    }

    #[test]
    fn test_is_permutation_primes() {
        // 317 and 371 are both prime and permutations
        assert!(is_permutation(317, 371));
    }

    #[test]
    fn test_prime_list() {
        let primes = prime_list(10, 25);
        // Primes between 10 and 25: 11, 13, 17, 19, 23
        assert!(primes.len() >= 4);
        assert!(primes.contains(&11));
        assert!(primes.contains(&13));
    }

    #[test]
    fn test_solve_produces_output() {
        let result = solve();
        assert!(!result.is_empty());
        // Result should be concatenation of three numbers
        assert!(result.len() >= 9);
    }
}

pe_utils::pe_main!();
