// Cuboid Route
// https://projecteuler.net/problem=86

use pe_lib::is_perfect_square;

fn solve() -> u64 {
    for m in 1..10000 {
        let mut count = 0;
        for a in 1..=m {
            for b in 1..=m {
                for c in 1..=m {
                    let d1_sq = a * a + (b + c) * (b + c);
                    let d2_sq = b * b + (a + c) * (a + c);
                    let d3_sq = c * c + (a + b) * (a + b);
                    if is_perfect_square(d1_sq) || is_perfect_square(d2_sq) || is_perfect_square(d3_sq) {
                        count += 1;
                    }
                }
            }
        }
        if count >= 1_000_000 {
            return m as u64;
        }
    }
    0
}

pe_utils::pe_main!();
