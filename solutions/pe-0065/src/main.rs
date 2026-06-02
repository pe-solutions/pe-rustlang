// Convergents of e
// https://projecteuler.net/problem=65

use pe_lib::Rational;

fn solve() -> u64 {
    let mut continued_fraction = vec![2];
    for k in 1..50 {
        continued_fraction.push(2 * k as i64);
        continued_fraction.push(1);
    }

    let mut numerator = Rational::new(1, 1);
    for &cf in continued_fraction.iter().rev() {
        numerator = Rational::new(cf, 1) + numerator.recip_custom();
    }

    numerator.numerator.abs().to_string().chars().map(|c| (c as u64) - 48).sum()
}

trait RationalExt {
    fn recip_custom(&self) -> Rational;
}

impl RationalExt for Rational {
    fn recip_custom(&self) -> Rational {
        Rational::new(self.denominator, self.numerator)
    }
}

pe_utils::pe_main!();
