// Ordered fractions
// https://projecteuler.net/problem=71

// Implementation of a Custom Rational instead of leveraging a ready for use crate - Design choice ;-)

use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord)]
pub struct Rational {
    numerator: i64,
    denominator: i64,
}

impl Rational {
    fn new(numerator: i64, denominator: i64) -> Self {
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

        Rational::new(self.numerator * multiplier / self.denominator +
                      other.numerator * multiplier / other.denominator,
                      multiplier)
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
        Rational::new(self.numerator * other.numerator,
                      self.denominator * other.denominator)
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
        self *
        Rational {
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
        (self.numerator * other.denominator).partial_cmp(&(self.denominator * other.numerator))
    }
}

impl<T: Into<i64>> From<T> for Rational {
    fn from(value: T) -> Self {
        Rational::new(value.into(), 1)
    }
}

fn main() {
     let start = std::time::Instant::now();
     
     println!();
     
     for d in (1..=1_000_000).rev() {
         let f = Rational::new(3, 7) - Rational::new(1, 7 * d);
         let n = f.numerator;
         
         if Rational::new(n, d) < Rational::new(3, 7) {
            let answer = n;
            println!("\nProject Euler #71\nAnswer: {}", answer);

            break;
         }
     }
     
    let duration = start.elapsed();

    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
 