// Lexicographic Permutations
// https://projecteuler.net/problem=24

use itertools::Itertools;

 fn get_millionth_permutation() -> Option<String> {
    if let Some(permutation) = (0..10_usize).permutations(10).nth(999_999) {
        Some(permutation.into_iter().map(|c| c.to_string()).collect())
    } else {
        None
    }
}

fn solve() -> String {
    get_millionth_permutation().expect("no permutation found")
}

pe_utils::pe_main!();
