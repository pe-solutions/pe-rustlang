// Pandigital Fibonacci ends
// https://projecteuler.net/problem=104

use num::bigint::BigInt;
use num::ToPrimitive;
use pe_lib::is_pandigital;

fn solve() -> u32 {
    let mut fib_prev = BigInt::from(0u32);
    let mut fib_curr = BigInt::from(1u32);
    let mut last_9_prev = 0u64;
    let mut last_9_curr = 1u64;
    let mut index = 1u32;
    const MOD_9: u64 = 1_000_000_000;

    loop {
        let fib_next: BigInt = &fib_prev + &fib_curr;
        let last_9_next = (last_9_prev + last_9_curr) % MOD_9;

        fib_prev = fib_curr;
        fib_curr = fib_next.clone();
        last_9_prev = last_9_curr;
        last_9_curr = last_9_next;
        index += 1;

        // Check last 9 digits first (cheap operation)
        if is_pandigital(&format!("{:09}", last_9_next)) {
            // Only format full number if last 9 digits are pandigital
            let fib_str = fib_next.to_string();
            // Check first 9 digits (if number is long enough)
            if fib_str.len() >= 9 && is_pandigital(&fib_str[0..9]) {
                return index;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pandigital_basic() {
        assert!(is_pandigital("123456789"));
        assert!(!is_pandigital("123456780"));
    }

    #[test]
    fn test_bigint_modulo() {
        let big = BigInt::from(123456789u64);
        let result = big % 1_000_000_000u64;
        assert_eq!(format!("{}", result), "123456789");
    }

    #[test]
    fn test_bigint_addition() {
        let a = BigInt::from(1u32);
        let b = BigInt::from(2u32);
        let sum = &a + &b;
        assert_eq!(sum, BigInt::from(3u32));
    }

    #[test]
    fn test_fibonacci_growth() {
        let mut fib_prev = BigInt::from(0u32);
        let mut fib_curr = BigInt::from(1u32);
        let fib_next = &fib_prev + &fib_curr;
        assert_eq!(fib_next, BigInt::from(1u32));

        fib_prev = fib_curr;
        fib_curr = fib_next;
        let fib_next2 = &fib_prev + &fib_curr;
        assert_eq!(fib_next2, BigInt::from(2u32));
    }

    #[test]
    fn test_solve_produces_index() {
        let result = solve();
        assert!(result > 0);
        assert!(result > 1000);  // Index should be reasonably large
    }
}

pe_utils::pe_main!();
