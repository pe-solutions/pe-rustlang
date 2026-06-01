// Distinct Powers
// https://projecteuler.net/problem=29

use num_bigint::BigUint;

fn solve() -> usize {
    let mut arr = Vec::new();
    for i in 2..=100 {
        for j in 2..=100 {
            arr.push(BigUint::from(i as u32).pow(j));
        }
    }
    arr.sort();
    arr.dedup();
    arr.len()
}

pe_utils::pe_main!();
