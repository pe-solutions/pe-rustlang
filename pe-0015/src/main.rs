// Lattice Paths
// https://projecteuler.net/problem=15

use num::integer::binomial;

fn solve() -> u64 {
    binomial(40u64, 20u64)
}

fn main() {
    pe_utils::run(15, solve);
}
