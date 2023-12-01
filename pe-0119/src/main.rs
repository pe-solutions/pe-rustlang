// Digit Power Sum
// https://projecteuler.net/problem=119

fn calculate_digit_sum(mut number: u64) -> u64 {
    let mut digit_sum = 0;

    while number > 0 {
        digit_sum += number % 10;
        number /= 10;
    }

    digit_sum
}

fn calculate_digit_power_sums(n: usize) -> u64 {
    let mut results: Vec<u64> = Vec::new();

    for base in 2u64..=70 {
        for power in 2u32..=10 {
            let number = base.pow(power);
            let digit_sum = calculate_digit_sum(number);

            if base == digit_sum {
                results.push(number);
            }
        }
    }

    results.sort();

    results[n - 1]
}

fn main() {
    let start = std::time::Instant::now();

    let answer = calculate_digit_power_sums(30);

    let duration = start.elapsed();

    println!("\nProject Euler #119\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_digit_power_sums_2() {
        assert_eq!(calculate_digit_power_sums(2), 512);
    }

    #[test]
    fn test_calculate_digit_power_sums_10() {
        assert_eq!(calculate_digit_power_sums(10), 614656);
    }
}
