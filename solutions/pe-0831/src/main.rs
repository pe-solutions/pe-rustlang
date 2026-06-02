// Triple Product
// https://projecteuler.net/problem=831

use num_bigint::BigInt;
use num_traits::{Num, Pow};

fn g_m(m: BigInt) -> BigInt {
    // Polynomial interpolation
    let numerator = BigInt::from(81) * m.clone().pow(5u32) + BigInt::from(153*5) * m.pow(4u32);
    let result = numerator / BigInt::from(40);
    
    result
}

fn pe0831() -> BigInt {
    let g_m_result = g_m(BigInt::from(142857));

    let g_str7 = g_m_result.to_str_radix(7);

    let g_digits = &g_str7.chars().collect::<Vec<char>>()[0..10];
    let g_string = g_digits.iter().collect::<String>();

    let result = BigInt::from_str_radix(&g_string, 10).unwrap();

    result
}

fn solve() -> num_bigint::BigInt {
    pe0831()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_g_m_small() {
        let result = g_m(BigInt::from(10));
        assert!(result > BigInt::from(0));
    }

    #[test]
    fn test_bigint_power() {
        let val = BigInt::from(2).pow(3u32);
        assert_eq!(val, BigInt::from(8));
    }

    #[test]
    fn test_base_conversion() {
        let num = BigInt::from(49); // 100 in base 7
        let base7 = num.to_str_radix(7);
        assert_eq!(base7, "100");
    }

    #[test]
    fn test_pe0831_produces_result() {
        let result = pe0831();
        assert!(result > BigInt::from(0));
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > BigInt::from(0));
    }
}

pe_utils::pe_main!();
