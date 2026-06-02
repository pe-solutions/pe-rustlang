// Digit Cancelling Fractions
// https://projecteuler.net/problem=33

use pe_lib::Rational;

fn solve() -> i64 {
    let mut answer = Rational::new(1, 1);
    for b in 1i64..10 {
        for c in (b + 1)..10 {
            let a_numer = 9 * b * c;
            let a_denom = 10 * c - b;
            if a_numer % a_denom == 0 && a_numer < 10 * a_denom {
                answer = answer * Rational::new(b, c);
            }
            let a_numer = 9 * b * c;
            let a_denom = 10 * b - c;
            if a_numer % a_denom == 0 && a_numer < 10 * a_denom {
                answer = answer * Rational::new(b, c);
            }
        }
    }
    answer.denominator
}

pe_utils::pe_main!();
