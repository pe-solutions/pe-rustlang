// Arithmetic Expressions
// https://projecteuler.net/problem=93

use std::collections::HashSet;

fn eval_all(nums: &[f64]) -> HashSet<u64> {
    let mut results = HashSet::new();
    if nums.len() == 1 {
        if nums[0] > 0.0 && nums[0] == nums[0].floor() {
            results.insert(nums[0] as u64);
        }
        return results;
    }

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            let a = nums[i];
            let b = nums[j];
            let mut new_nums = Vec::new();
            for (k, &n) in nums.iter().enumerate() {
                if k != i && k != j {
                    new_nums.push(n);
                }
            }

            for op_result in &[a + b, a - b, a * b, a / b] {
                if op_result.is_finite() && *op_result >= 0.0 {
                    let mut test_nums = new_nums.clone();
                    test_nums.push(*op_result);
                    let sub_results = eval_all(&test_nums);
                    results.extend(sub_results);
                }
            }
        }
    }
    results
}

fn count_consecutive_sequence(results: &HashSet<u64>) -> u64 {
    let mut consecutive = 0;
    let mut n = 1;
    while results.contains(&n) {
        consecutive += 1;
        n += 1;
    }
    consecutive
}

fn solve() -> u64 {
    let mut best_consecutive = 0;
    let mut best_num = 0;

    for a in 1..=9 {
        for b in a + 1..=9 {
            for c in b + 1..=9 {
                for d in c + 1..=9 {
                    let nums = vec![a as f64, b as f64, c as f64, d as f64];
                    let results = eval_all(&nums);
                    let consecutive = count_consecutive_sequence(&results);

                    if consecutive > best_consecutive {
                        best_consecutive = consecutive;
                        best_num = (a * 1000 + b * 100 + c * 10 + d) as u64;
                    }
                }
            }
        }
    }
    best_num
}

pe_utils::pe_main!();
