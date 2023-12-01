// Concealed Square
// https://projecteuler.net/problem=206

fn calculate_square_root_upper_bound() -> u64 {
    ((192_939_495_969_798_999_0u64 as f64).sqrt() / 10.0) as u64 * 10
}

fn calculate_square_root_lower_bound() -> u64 {
    ((102_030_405_060_708_090_0u64 as f64).sqrt() / 10.0) as u64 * 10
}

fn has_concealed_square_pattern(candidate: u64) -> bool {
    let mut candidate = candidate / 100;

    for digit in (1..=9).rev() {
        if candidate % 10 != digit {
            return false;
        }
        candidate /= 100;
    }

    true
}

fn find_concealed_square() -> u64 {
    let max = calculate_square_root_upper_bound();
    let min = calculate_square_root_lower_bound();

    for i in (min..=max).step_by(10) {
        let candidate = i * i;

        if has_concealed_square_pattern(candidate) {
            return i;
        }
    }

    0 // Default return if no concealed square is found
}

fn main() {
    let timer = std::time::Instant::now();

    let answer = find_concealed_square();

    let elapsed_time = timer.elapsed();

    println!("\nProject Euler #206\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", elapsed_time.as_millis());
}
