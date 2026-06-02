// Amicable Chains
// https://projecteuler.net/problem=95

use pe_lib::sum_proper_divisors;

fn chain_length(n: u64, limit: u64) -> u64 {
    let mut seen = std::collections::HashSet::new();
    let mut current = n;
    let mut length = 0;

    while !seen.contains(&current) && current > 0 && current < limit {
        seen.insert(current);
        current = sum_proper_divisors(current);
        length += 1;
    }

    if seen.contains(&current) && seen.contains(&n) {
        length
    } else {
        0
    }
}

fn solve() -> u64 {
    let limit = 1_000_000;
    let mut best_chain_len = 0;
    let mut best_num = 0;

    for n in 1..limit {
        let len = chain_length(n, limit);
        if len > best_chain_len {
            best_chain_len = len;
            best_num = n;
        }
    }

    let mut chain = vec![best_num];
    let mut current = sum_proper_divisors(best_num);
    while current != best_num {
        chain.push(current);
        current = sum_proper_divisors(current);
    }
    chain.sort();
    *chain.first().unwrap_or(&0)
}

pe_utils::pe_main!();
