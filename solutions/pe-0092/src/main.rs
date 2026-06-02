// Square Digit Chains
// https://projecteuler.net/problem=92

use std::collections::HashMap;
use pe_lib::digit_sum_sq as sum_of_digit_squares;

// Determines if the number `n` eventually ends up at 89 or 1,
// using memoization to cache previously computed results.
fn ends_in_89(n: u64, memo: &mut HashMap<u64, bool>) -> bool {
    let mut current = n;
    while current != 1 && current != 89 {
        current = sum_of_digit_squares(current);
    }
    
    let result = current == 89;
    
    memo.insert(n, result);
    
    result
}

fn solve() -> u32 {
    let limit = 10_000_000;
    let mut count = 0;
    let mut memo = HashMap::new();
    for i in 1..limit {
        if ends_in_89(i, &mut memo) { count += 1; }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_chains() {
        // 1 ends at 1 (stays at 1)
        let mut memo = HashMap::new();
        assert!(!ends_in_89(1, &mut memo));

        // 89 is already 89
        assert!(ends_in_89(89, &mut memo));
    }

    #[test]
    fn test_digit_sum_squares() {
        assert_eq!(sum_of_digit_squares(44), 32);   // 4^2 + 4^2 = 32
        assert_eq!(sum_of_digit_squares(85), 89);   // 8^2 + 5^2 = 64+25 = 89
    }

    #[test]
    fn test_chain_property() {
        let mut memo = HashMap::new();
        // 2 -> 4 -> 16 -> 37 -> 58 -> 89
        assert!(ends_in_89(2, &mut memo));
    }

    #[test]
    fn test_solve_produces_count() {
        let result = solve();
        assert!(result > 0);
        assert!(result < 10_000_000);
    }
}

pe_utils::pe_main!();
