// Factorial Digit Sum
// https://projecteuler.net/problem=20

use pe_lib::factorial;

fn solve() -> u64 {
    let fact = factorial(100);
    fact.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_growth() {
        let f5 = factorial(5);
        let f6 = factorial(6);
        assert_eq!(&f6 / &f5, 6u32.into());
    }

    #[test]
    fn test_digit_sum_small_factorials() {
        let fact5 = factorial(5); // 120
        let digit_sum: u64 = fact5.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .sum();
        assert_eq!(digit_sum, 1 + 2 + 0);
    }

    #[test]
    fn test_solve_produces_output() {
        let result = solve();
        assert!(result > 0);
        // 100! has many digits, so digit sum should be substantial
        assert!(result > 100);
    }

    #[test]
    fn test_factorial_digit_sum_consistency() {
        let fact10 = factorial(10); // 3,628,800
        let digit_sum: u64 = fact10.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .sum();
        assert_eq!(digit_sum, 3 + 6 + 2 + 8 + 8 + 0 + 0);
    }
}

pe_utils::pe_main!();
