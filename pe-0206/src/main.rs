// Concealed Square
// https://projecteuler.net/problem=206

fn calculate_square_root_upper_bound() -> u64 {
    ((1929394959697989990u64 as f64).sqrt() / 10.0) as u64 * 10
}

fn calculate_square_root_lower_bound() -> u64 {
    ((1020304050607080900u64 as f64).sqrt() / 10.0) as u64 * 10
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

fn main() {
    let timer = std::time::Instant::now();

    let max = calculate_square_root_upper_bound();
    let min = calculate_square_root_lower_bound();

    let mut answer = 0;

    for i in (min..=max).step_by(10) {
        let candidate = i * i;

        if has_concealed_square_pattern(candidate) {
            answer = i;
            break;
        }
    }

    let elapsed_time = timer.elapsed();

    println!("\nProject Euler #206");
    println!("Answer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", elapsed_time.as_millis());
}
