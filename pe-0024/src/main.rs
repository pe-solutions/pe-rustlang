// Lexicographic Permutations
// https://projecteuler.net/problem=24

use itertools::Itertools;

fn find_millionth_permutation() -> Result<String, &'static str> {
    let permutation = (0..10)
        .permutations(10)
        .nth(1_000_000 - 1) // 1-based indexing, so subtract 1
        .ok_or("Error: Could not find the millionth lexicographic permutation.")?;

    let permutation_str = permutation.into_iter().map(|x| x.to_string()).collect::<String>();
    
    Ok(permutation_str)
}

fn main() {
    let start_time = std::time::Instant::now();
    
    let answer = find_millionth_permutation().unwrap_or_default();
    
    let duration = start_time.elapsed();
    
    println!("\nProject Euler #24\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
