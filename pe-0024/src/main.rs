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

fn main() {
    let start = std::time::Instant::now();
    
    println!(
        "\nProject Euler #24\nAnswer: {:?}\nElapsed time: {:?} milliseconds.",
        get_millionth_permutation(),
        start.elapsed().as_millis()
    );
}