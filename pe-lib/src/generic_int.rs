use std::ops::{Add, Div, Mul, Rem};

/// Generic trait for integer operations used in digit manipulation and other algorithms.
pub trait GenericInt: Copy + Ord + Sized + std::fmt::Display {
    /// Convert from u64 (panics if overflow for smaller types)
    fn from_u64(n: u64) -> Self;

    /// Convert from u128
    fn from_u128(n: u128) -> Self;

    /// Constant 10 for digit operations
    fn ten() -> Self;

    /// Constant 1
    fn one() -> Self;

    /// Constant 0
    fn zero() -> Self;
}

impl GenericInt for u64 {
    fn from_u64(n: u64) -> Self {
        n
    }

    fn from_u128(n: u128) -> Self {
        n as u64
    }

    fn ten() -> Self {
        10
    }

    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }
}

impl GenericInt for u128 {
    fn from_u64(n: u64) -> Self {
        n as u128
    }

    fn from_u128(n: u128) -> Self {
        n
    }

    fn ten() -> Self {
        10
    }

    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }
}

/// Generic digit sum that works for any integer type implementing GenericInt.
pub fn digit_sum_generic<T: GenericInt + Add<Output = T> + Rem<Output = T> + Div<Output = T>>(n: T) -> T {
    let mut sum = T::zero();
    let mut num = n;
    while num > T::zero() {
        sum = sum + (num % T::ten());
        num = num / T::ten();
    }
    sum
}

/// Generic digit sum of squares that works for any integer type implementing GenericInt.
pub fn digit_sum_sq_generic<T: GenericInt + Add<Output = T> + Mul<Output = T> + Rem<Output = T> + Div<Output = T>>(n: T) -> T {
    let mut sum = T::zero();
    let mut num = n;
    while num > T::zero() {
        let digit = num % T::ten();
        sum = sum + (digit * digit);
        num = num / T::ten();
    }
    sum
}

/// Generic reverse digits that works for any integer type implementing GenericInt.
pub fn reverse_digits_generic<T: GenericInt + Add<Output = T> + Mul<Output = T> + Rem<Output = T> + Div<Output = T>>(n: T) -> T {
    let mut reversed = T::zero();
    let mut num = n;
    while num > T::zero() {
        reversed = (reversed * T::ten()) + (num % T::ten());
        num = num / T::ten();
    }
    reversed
}

/// Generic palindrome check that works for any integer type implementing GenericInt + PartialEq.
pub fn is_palindrome_generic<T: GenericInt + Add<Output = T> + Mul<Output = T> + Rem<Output = T> + Div<Output = T> + PartialEq>(n: T) -> bool {
    n == reverse_digits_generic(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_sum_u64() {
        assert_eq!(digit_sum_generic::<u64>(123), 6);
        assert_eq!(digit_sum_generic::<u64>(999), 27);
    }

    #[test]
    fn test_digit_sum_u128() {
        assert_eq!(digit_sum_generic::<u128>(123), 6);
        assert_eq!(digit_sum_generic::<u128>(999), 27);
    }

    #[test]
    fn test_reverse_digits_u64() {
        assert_eq!(reverse_digits_generic::<u64>(123), 321);
        assert_eq!(reverse_digits_generic::<u64>(1000), 1);
    }

    #[test]
    fn test_reverse_digits_u128() {
        assert_eq!(reverse_digits_generic::<u128>(123), 321);
        assert_eq!(reverse_digits_generic::<u128>(1000), 1);
    }

    #[test]
    fn test_is_palindrome_u64() {
        assert!(is_palindrome_generic::<u64>(121));
        assert!(!is_palindrome_generic::<u64>(123));
    }

    #[test]
    fn test_is_palindrome_u128() {
        assert!(is_palindrome_generic::<u128>(121));
        assert!(!is_palindrome_generic::<u128>(123));
    }
}
