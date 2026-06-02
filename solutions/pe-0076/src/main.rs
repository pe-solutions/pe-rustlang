// Counting Summations
// https://projecteuler.net/problem=76

use pe_lib::count_partitions;

fn solve() -> usize {
    count_partitions(100) - 1
}

pe_utils::pe_main!();
