// Pandigital Prime
// https://projecteuler.net/problem=41

extern crate itertools;

use itertools::Itertools;
use pe_lib::is_prime_trial as is_prime;

fn solve() -> u64 {
    let digits = vec!['7', '6', '5', '4', '3', '2', '1'];
    let permutations = digits.iter().permutations(digits.len());
    permutations
        .filter_map(|perm| {
            let n: u64 = perm.into_iter().collect::<String>().parse().unwrap();
            if is_prime(n) { Some(n) } else { None }
        })
        .max()
        .expect("no prime permutation found")
}

pe_utils::pe_main!();
