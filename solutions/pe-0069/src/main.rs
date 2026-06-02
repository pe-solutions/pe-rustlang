// Totient Maximum
// https://projecteuler.net/problem=69

use pe_lib::totient;

fn solve() -> u64 {
    let mut max_ratio = 0.0;
    let mut result = 0u64;

    for n in 2..=1_000_000u64 {
        let phi = totient(n);
        let ratio = n as f64 / phi as f64;
        if ratio > max_ratio {
            max_ratio = ratio;
            result = n;
        }
    }

    result
}

pe_utils::pe_main!();
