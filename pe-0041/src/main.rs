// Pandigital Prime
// https://projecteuler.net/problem=41

extern crate itertools;

use itertools::Itertools;

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true}

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
