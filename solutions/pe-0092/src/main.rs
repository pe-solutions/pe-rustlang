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

pe_utils::pe_main!();
