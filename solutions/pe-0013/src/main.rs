// Large sum
// https://projecteuler.net/problem=13

use num_bigint::BigUint;
use pe_utils::read_lines;

fn large_sum(arr: Vec<String>) -> Result<BigUint, num_bigint::ParseBigIntError> {
    let mut sum = BigUint::from(0u32);
    for line in arr {
        let num = line.parse::<BigUint>()?;
        sum += num;
    }
    Ok(sum)
}

fn solve() -> u64 {
    let arr = read_lines("data/numbers.txt").expect("failed to read data/numbers.txt");
    let sum = large_sum(arr).expect("failed to parse numbers");
    sum.to_string()[..10].parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn test_large_sum_basic() {
        let numbers = vec!["123".to_string(), "456".to_string(), "789".to_string()];
        let result = large_sum(numbers).unwrap();
        assert_eq!(result, BigUint::from(1368u32));
    }

    #[test]
    fn test_large_sum_big_integers() {
        let numbers = vec![
            "12345678901234567890".to_string(),
            "98765432109876543210".to_string(),
        ];
        let result = large_sum(numbers).unwrap();
        assert!(result.to_string().len() > 19);
    }

    #[test]
    fn test_large_sum_monotonic() {
        let nums1 = vec!["100".to_string(), "200".to_string()];
        let nums2 = vec!["100".to_string(), "200".to_string(), "300".to_string()];
        let sum1 = large_sum(nums1).unwrap();
        let sum2 = large_sum(nums2).unwrap();
        assert!(sum2 > sum1);
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > 0);
    }
}

pe_utils::pe_main!();
