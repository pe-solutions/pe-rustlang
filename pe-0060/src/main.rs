// Prime Pair Sets
// https://projecteuler.net/problem=60


mod prime_utils;
use crate::prime_utils::find_prime_set;

fn solve() -> u64 {
    find_prime_set().expect("no prime set found").iter().sum()
}

fn main() {
    pe_utils::run(60, solve);
}
