// Digit Power Sum
// https://projecteuler.net/problem=119

use pe_lib::digit_sum as calculate_digit_sum;

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

fn solve() -> u64 {
    calculate_digit_power_sums(30)
}

pe_utils::pe_main!();
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_sum_basic() {
        assert_eq!(calculate_digit_sum(18), 1 + 8);
        assert_eq!(calculate_digit_sum(512), 5 + 1 + 2);
        assert_eq!(calculate_digit_sum(27), 2 + 7);
    }

    #[test]
    fn test_power_calculation() {
        // 2^9 = 512, digit sum = 8
        assert_eq!(calculate_digit_sum(2u64.pow(9)), 8);
        // 3^3 = 27, digit sum = 9
        assert_eq!(calculate_digit_sum(3u64.pow(3)), 9);
    }

    #[test]
    fn test_calculate_digit_power_sums_2() {
        assert_eq!(calculate_digit_power_sums(2), 512);
    }

    #[test]
    fn test_calculate_digit_power_sums_10() {
        assert_eq!(calculate_digit_power_sums(10), 614656);
    }

    #[test]
    fn test_solve_produces_output() {
        let result = solve();
        assert!(result > 0);
        assert!(result > 1000);
    }
}

