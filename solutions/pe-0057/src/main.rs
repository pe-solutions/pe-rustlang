// Square Root Convergents
// https://projecteuler.net/problem=57

use pe_lib::Rational;

fn solve() -> u64 {
    let mut count = 0;
    let mut r = Rational::new(3, 2);

    for _ in 1..1000 {
        // Reciprocal: swap numerator and denominator
        let recip = Rational::new(r.denominator, r.numerator);
        r = Rational::new(1, 2) + recip;

        let num_digits = r.numerator.abs().to_string().len();
        let den_digits = r.denominator.to_string().len();

        if num_digits > den_digits {
            count += 1;
        }
    }

    count
}

pe_utils::pe_main!();
