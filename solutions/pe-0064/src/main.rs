// Odd Period Square Roots
// https://projecteuler.net/problem=64

use pe_lib::{isqrt, is_perfect_square};

fn solve() -> u64 {
    let mut count = 0;

    for n in 2..=10000u64 {
        if is_perfect_square(n) {
            continue;
        }

        let m0 = 0;
        let d0 = 1;
        let a0 = isqrt(n);

        let mut m = d0 * a0 - m0;
        let mut d = (n - m * m) / d0;
        let mut a = (a0 + m) / d;

        let mut period = 0;
        while a != 2 * a0 {
            m = d * a - m;
            d = (n - m * m) / d;
            a = (a0 + m) / d;
            period += 1;
        }

        if period % 2 == 1 {
            count += 1;
        }
    }

    count
}

pe_utils::pe_main!();
