// Nth Digit of Reciprocals
// https://projecteuler.net/problem=820

use pe_lib::mod_pow;

fn calculate_nth_digit_sum(n: u64) -> u64 {
    (1..=n)
        .map(|i| mod_pow(10, n - 1, i) * 10 / i)
        .sum()
}

fn solve() -> u64 {
    calculate_nth_digit_sum(10_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_small() {
        let result = calculate_nth_digit_sum(10);
        assert!(result > 0);
    }

    #[test]
    fn test_calculate_monotonic() {
        let a = calculate_nth_digit_sum(10);
        let b = calculate_nth_digit_sum(20);
        assert!(b >= a);
    }

    #[test]
    fn test_mod_pow_usage() {
        let val = mod_pow(10, 5, 7);
        assert!(val < 7);
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > 0);
    }
}

pe_utils::pe_main!();
