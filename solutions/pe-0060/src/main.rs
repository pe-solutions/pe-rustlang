// Prime Pair Sets
// https://projecteuler.net/problem=60


mod prime_utils;
use crate::prime_utils::find_prime_set;

fn solve() -> u64 {
    find_prime_set().expect("no prime set found").iter().sum()
}

pe_utils::pe_main!();
