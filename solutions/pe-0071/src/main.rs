// Ordered fractions
// https://projecteuler.net/problem=71

use pe_lib::Rational;

fn solve() -> i64 {
    for d in (1..=1_000_000).rev() {
        let f = Rational::new(3, 7) - Rational::new(1, 7 * d);
        let n = f.numerator;
        if Rational::new(n, d) < Rational::new(3, 7) {
            return n;
        }
    }
    panic!("no answer found")
}

pe_utils::pe_main!();
