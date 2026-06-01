// Combinatoric Selections
// https://projecteuler.net/problem=53

fn solve() -> u32 {
    const THRESHOLD: u128 = 1_000_000;
    (1u32..=100)
        .filter_map(|n| {
            let mut binom = 1u128;
            (0..=n / 2).find_map(|r| {
                if binom > THRESHOLD {
                    let valid_count = if n % 2 == 0 && r == n / 2 { 1 } else { n - 2 * r + 1 };
                    Some(valid_count)
                } else {
                    binom = binom * (n - r) as u128 / (r + 1) as u128;
                    None
                }
            })
        })
        .sum()
}

pe_utils::pe_main!();
