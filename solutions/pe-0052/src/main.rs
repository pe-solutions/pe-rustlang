// Permuted Multiples
// https://projecteuler.net/problem=52

use pe_lib::is_permutation;

fn solve() -> u64 {
    for x in 1..10_000_000u64 {
        if is_permutation(x, 2 * x)
            && is_permutation(x, 3 * x)
            && is_permutation(x, 4 * x)
            && is_permutation(x, 5 * x)
            && is_permutation(x, 6 * x)
        {
            return x;
        }
    }
    0
}

pe_utils::pe_main!();
