// Largest Prime Factor
// https://projecteuler.net/problem/3

use pe_lib::largest_prime_factor;

fn solve() -> u64 {
    largest_prime_factor(600_851_475_143)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factorization_small() {
        assert!(is_prime(13));
        assert!(!is_prime(15));
    }

    #[test]
    fn test_largest_prime_factor_small() {
        let factors = prime_factors(13);
        assert_eq!(factors.len(), 1);
        assert_eq!(factors[0].0, 13);
    }

    #[test]
    fn test_solve_produces_output() {
        assert!(solve() > 0);
    }
}
pe_utils::pe_main!();
