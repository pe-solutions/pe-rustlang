// Lattice Paths
// https://projecteuler.net/problem=15

use num::integer::binomial;

fn solve() -> u64 {
    binomial(40u64, 20u64)
}

pe_utils::pe_main!();
