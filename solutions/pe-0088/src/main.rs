// Product-sum Numbers
// https://projecteuler.net/problem=88

fn solve() -> u64 {
    let max_k = 12000;
    let mut min_ps = vec![u64::MAX; max_k + 1];

    fn generate(prod: u64, sum: u64, n: u64, start: u64, max_k: usize, min_ps: &mut Vec<u64>) {
        if n > start {
            let k = (prod - sum + n) as usize;
            if k <= max_k && prod < min_ps[k] {
                min_ps[k] = prod;
            }
        }

        for i in start..=min(prod - sum + n, 2 * max_k as u64) {
            if prod / i >= i && prod % i == 0 {
                generate(prod, sum + i, n + 1, i, max_k, min_ps);
            }
        }
    }

    for n in 2..max_k as u64 {
        generate(n, 0, 0, 2, max_k, &mut min_ps);
    }

    let mut sum = 0u64;
    let mut seen = std::collections::HashSet::new();
    for k in 2..=max_k {
        if min_ps[k] != u64::MAX && seen.insert(min_ps[k]) {
            sum += min_ps[k];
        }
    }
    sum
}

fn min(a: u64, b: u64) -> u64 {
    if a < b { a } else { b }
}

pe_utils::pe_main!();
