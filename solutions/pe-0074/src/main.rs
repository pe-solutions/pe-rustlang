// Digit Factorial Chains
// https://projecteuler.net/problem=74

use pe_lib::digit_factorial_sum;

fn solve() -> u64 {
    let mut count = 0;
    for n in 1..1_000_000 {
        let mut seen = std::collections::HashSet::new();
        let mut m = n;
        while !seen.contains(&m) && m != 169 {
            seen.insert(m);
            m = digit_factorial_sum(m);
        }
        if seen.len() == 60 {
            count += 1;
        }
    }
    count
}

pe_utils::pe_main!();
