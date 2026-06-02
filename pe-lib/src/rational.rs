use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

// Rational number implementation supporting i64
// Maintains fractions in lowest terms via GCD normalization

/// Rational number struct representing a fraction in lowest terms
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord)]
pub struct Rational {
    pub numerator: i64,
    pub denominator: i64,
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

impl Rational {
    /// Create a new rational number, automatically reduced to lowest terms
    pub fn new(numerator: i64, denominator: i64) -> Self {
        let divisor = gcd(numerator, denominator);
        Rational {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
        }
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let multiplier = lcm(self.denominator, other.denominator);
        Rational::new(
            self.numerator * multiplier / self.denominator +
                other.numerator * multiplier / other.denominator,
            multiplier,
        )
    }
}

impl AddAssign for Rational {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + -other
    }
}

impl SubAssign for Rational {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Rational::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }
}

impl MulAssign for Rational {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * Rational {
            numerator: other.denominator,
            denominator: other.numerator,
        }
    }
}

impl DivAssign for Rational {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self {
        Rational {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.numerator * other.denominator)
            .partial_cmp(&(self.denominator * other.numerator))
    }
}

impl<T: Into<i64>> From<T> for Rational {
    fn from(value: T) -> Self {
        Rational::new(value.into(), 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rational_new_reduces() {
        let r = Rational::new(2, 4);
        assert_eq!(r.numerator, 1);
        assert_eq!(r.denominator, 2);
    }

    #[test]
    fn test_rational_add() {
        let r1 = Rational::new(1, 2);
        let r2 = Rational::new(1, 3);
        let result = r1 + r2;
        assert_eq!(result.numerator, 5);
        assert_eq!(result.denominator, 6);
    }

    #[test]
    fn test_rational_sub() {
        let r1 = Rational::new(3, 4);
        let r2 = Rational::new(1, 4);
        let result = r1 - r2;
        assert_eq!(result.numerator, 1);
        assert_eq!(result.denominator, 2);
    }

    #[test]
    fn test_rational_mul() {
        let r1 = Rational::new(2, 3);
        let r2 = Rational::new(3, 4);
        let result = r1 * r2;
        assert_eq!(result.numerator, 1);
        assert_eq!(result.denominator, 2);
    }

    #[test]
    fn test_rational_div() {
        let r1 = Rational::new(1, 2);
        let r2 = Rational::new(1, 3);
        let result = r1 / r2;
        assert_eq!(result.numerator, 3);
        assert_eq!(result.denominator, 2);
    }

    #[test]
    fn test_rational_cmp() {
        let r1 = Rational::new(1, 2);
        let r2 = Rational::new(2, 3);
        assert!(r1 < r2);
    }

    #[test]
    fn test_rational_from() {
        let r: Rational = 5i64.into();
        assert_eq!(r.numerator, 5);
        assert_eq!(r.denominator, 1);
    }

    #[test]
    fn test_rational_neg() {
        let r = Rational::new(3, 4);
        let neg_r = -r;
        assert_eq!(neg_r.numerator, -3);
        assert_eq!(neg_r.denominator, 4);
    }

    #[test]
    fn test_rational_assign_ops() {
        let mut r = Rational::new(1, 2);
        r += Rational::new(1, 4);
        assert_eq!(r.numerator, 3);
        assert_eq!(r.denominator, 4);
    }
}
