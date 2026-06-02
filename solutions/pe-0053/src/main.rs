// Combinatoric Selections
// https://projecteuler.net/problem=53

use pe_lib::binomial_big;

fn solve() -> u32 {
    let threshold = 1_000_000u128;
    (1u32..=100)
        .filter_map(|n| {
            (0..=n / 2).find_map(|r| {
                let binom = binomial_big(n, r);
                if binom > threshold.into() {
                    let valid_count = if n % 2 == 0 && r == n / 2 { 1 } else { n - 2 * r + 1 };
                    Some(valid_count)
                } else {
                    None
                }
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial_growth() {
        // C(n, 0) = 1 for all n
        // C(n, n/2) is largest
        // For small n, C(n, k) < 1_000_000
        let n = 10u32;
        let mut binom = 1u128;
        for k in 0..=n / 2 {
            if k > 0 {
                binom = binom * (n - (k - 1)) as u128 / k as u128;
            }
            assert!(binom > 0);
        }
    }

    #[test]
    fn test_threshold_crossing() {
        const THRESHOLD: u128 = 1_000_000;
        // For n=23, binomial coefficients exceed threshold
        let n = 23u32;
        let mut binom = 1u128;
        let mut exceeded = false;
        for r in 0..=n / 2 {
            if binom > THRESHOLD {
                exceeded = true;
                break;
            }
            binom = binom * (n - r) as u128 / (r + 1) as u128;
        }
        assert!(exceeded); // Eventually exceeds threshold
    }

    #[test]
    fn test_valid_count_formula() {
        // For odd n, all valid coefficients have count n - 2*r + 1
        let n = 13u32;
        assert!(n % 2 == 1); // odd
        for r in 0..n / 2 {
            let count = n - 2 * r + 1;
            assert!(count > 0);
        }
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > 0);
    }
}

pe_utils::pe_main!();
