// Counting Rectangles
// https://projecteuler.net/problem=85

fn solve() -> u64 {
    let target = 2_000_000;
    let mut best_area = 0;
    let mut best_diff = i64::MAX;

    for m in 1..=2000 {
        for n in m..=2000 {
            let count = (m * (m + 1) / 2) as i64 * (n * (n + 1) / 2) as i64;
            let diff = (count - target as i64).abs();
            if diff < best_diff {
                best_diff = diff;
                best_area = (m * n) as u64;
            }
            if count > target as i64 {
                break;
            }
        }
    }
    best_area
}

pe_utils::pe_main!();
