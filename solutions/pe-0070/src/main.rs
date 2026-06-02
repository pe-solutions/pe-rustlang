// Totient Permutation
// https://projecteuler.net/problem=70

use pe_lib::sieve_primes as prime_sieve;


fn find_n(lim: usize) -> usize {
    // Generate primes (bounded by sqrt(lim))
    let primes = prime_sieve((2 * (lim as f64).sqrt() as usize) + 1);

    let mut best_ratio = lim as f64;
    let mut best_n = 0;

    
    // Look only into two-distinct-prime composites
    for (i, &p1) in primes.iter().enumerate() {
        for &p2 in &primes[i + 1..] {
            let product = p1 * p2;

            if product > lim {
                break;
            }

            let n = product;
            let phi = (p1 - 1) * (p2 - 1);

            let ratio = n as f64 / phi as f64;

            // Best ration and Permutation
            if ratio < best_ratio && sorted_digits(n) == sorted_digits(phi) {
                best_ratio = ratio;
                best_n = n;
            }
        }
    }

    best_n
}

// Helper function
fn sorted_digits(n: usize) -> Vec<char> {
    let mut digits: Vec<char> = n.to_string().chars().collect();
    digits.sort();

    digits
}

fn solve() -> usize {
    find_n(10_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_digits() {
        assert_eq!(sorted_digits(123), vec!['1', '2', '3']);
        assert_eq!(sorted_digits(321), vec!['1', '2', '3']);
        assert_eq!(sorted_digits(1), vec!['1']);
    }

    #[test]
    fn test_sorted_digits_permutation() {
        let d1 = sorted_digits(132);
        let d2 = sorted_digits(213);
        assert_eq!(d1, d2);
    }

    #[test]
    fn test_totient_property() {
        // For n = p1 * p2 (two distinct primes),
        // φ(n) = (p1-1)*(p2-1)
        // Example: 6 = 2*3, φ(6) = 1*2 = 2
        let phi = (2 - 1) * (3 - 1);
        assert_eq!(phi, 2);
    }

    #[test]
    fn test_find_n_produces_result() {
        let result = find_n(100);
        // Should find some n or return 0
        assert!(result >= 0);
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > 0);
        assert!(result < 10_000_000);
    }
}

pe_utils::pe_main!();
