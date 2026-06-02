// Digit Sum Numbers
// https://projecteuler.net/problem=725

// Using Closed formula (cf. Pari-GP solution): 
//     [t1]$S(n) = \left(2\binom{n+8}{8} - \binom{10}{2}\right) \cdot (n-1) \cdot \frac{10^n-1}{9}$

use num_bigint::BigUint;
use num_traits::Pow;
use pe_lib::binomial_big;

fn s(n: u32) -> BigUint {
    let binomial1 = binomial_big(n + 8, 8);
    let binomial2 = binomial_big(10, 2);

    let term1 = BigUint::from(2u32) * binomial1 - binomial2;
    let term2 = BigUint::from(n - 1);
    let term3 = BigUint::from(10u32).pow(n) - BigUint::from(1u32);

    &term1 * &term2 * term3 / BigUint::from(9u32)
}

fn calculate_digit_sum_modulo() -> BigUint {
    let s_2020 = s(2020);
    s_2020 % BigUint::from(10u32).pow(16u32)
}

fn solve() -> num_bigint::BigUint {
    calculate_digit_sum_modulo()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial_computation() {
        let b = binomial_big(10, 2);
        assert_eq!(b, BigUint::from(45u32)); // C(10,2) = 45
    }

    #[test]
    fn test_s_function_small_values() {
        let s2 = s(2);
        assert!(s2 > BigUint::from(0u32));
    }

    #[test]
    fn test_power_calculation() {
        let power = BigUint::from(10u32).pow(3u32);
        assert_eq!(power, BigUint::from(1000u32));
    }

    #[test]
    fn test_s_grows_exponentially() {
        let s2 = s(2);
        let s3 = s(3);
        assert!(s3 > s2);
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > BigUint::from(0u32));
    }
}

pe_utils::pe_main!();
