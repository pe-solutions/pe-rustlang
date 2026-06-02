// Lattice Paths
// https://projecteuler.net/problem=15

use pe_lib::binomial_big;

fn solve() -> u64 {
    let result = binomial_big(40, 20);
    result.to_u64_digits().get(0).copied().unwrap_or(0)
}

pe_utils::pe_main!();
