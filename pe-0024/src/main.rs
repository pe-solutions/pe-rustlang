use itertools::join;

fn nth_permutation<T: Copy + Ord>(mut elements: Vec<T>, mut n: usize) -> Vec<T> {
    let mut result = Vec::new();
    let mut factorial = 1;

    for i in 1..elements.len() {
        factorial *= i;
    }

    while !elements.is_empty() {
        let index = n / factorial;
        result.push(elements.remove(index));
        n %= factorial;

        if !elements.is_empty() {
            factorial /= elements.len();
        }
    }

    result
}

fn main() {
    let start_time = std::time::Instant::now();

    let src = (0..=9).collect::<Vec<_>>();

    let permutation = nth_permutation(src, 1_000_000 - 1);

    let answer = join(permutation, "");

    let duration = start_time.elapsed();
    
    println!("\nProject Euler #24\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
