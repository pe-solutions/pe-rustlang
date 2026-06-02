// Counting Fractions in a Range
// https://projecteuler.net/problem=73

use pe_lib::gcd;

fn solve() -> u64 {
    let mut count = 0;
    for d in 2..=12000u64 {
        for n in (d/3 + 1)..(d/2) {
            if gcd(n, d) == 1 {
                count += 1;
            }
        }
    }
    count
}

pe_utils::pe_main!();
