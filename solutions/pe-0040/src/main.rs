// Champernowne's Constant
// https://projecteuler.net/problem=40

use pe_lib::digits;

fn solve() -> u64 {
    let c: Vec<u64> = (1..=250000)
        .flat_map(|x: u64| digits(x).iter().map(|&d| d as u64).collect::<Vec<_>>())
        .collect();
    c[0] * c[9] * c[99] * c[999] * c[9999] * c[99999] * c[999999]
}

pe_utils::pe_main!();
