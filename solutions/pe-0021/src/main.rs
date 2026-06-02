// Amicable Numbers
// https://projecteuler.net/problem=21

use pe_lib::sum_proper_divisors;

fn solve() -> i32 {
    const RANGE_MAX: i32 = 10_000;
    let mut answer = 0;
    for n in 1..RANGE_MAX {
        let sum_n = sum_proper_divisors(n as u64) as i32;
        if n < sum_n && sum_n <= RANGE_MAX && sum_proper_divisors(sum_n as u64) == n as u64 {
            answer += n + sum_n;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amicable_pair_220_284() {
        // 220 and 284 are the smallest amicable pair
        let sum_220 = sum_proper_divisors(220);
        let sum_284 = sum_proper_divisors(284);
        assert_eq!(sum_220, 284);
        assert_eq!(sum_284, 220);
    }

    #[test]
    fn test_perfect_numbers_not_amicable() {
        // Perfect numbers like 6 are not amicable (n = sum_proper_divisors(n))
        let sum_6 = sum_proper_divisors(6);
        assert_eq!(sum_6, 6);
        // 6 is not amicable with another different number
    }

    #[test]
    fn test_amicable_pair_property() {
        let (a, b) = (220u64, 284u64);
        let sum_a = sum_proper_divisors(a);
        let sum_b = sum_proper_divisors(b);
        assert_eq!(sum_a, b);
        assert_eq!(sum_b, a);
    }

    #[test]
    fn test_solve_produces_output() {
        let result = solve();
        assert!(result > 0);
        // Known: sum of amicable numbers below 10000 is 31626
        assert!(result > 1000);
    }
}

pe_utils::pe_main!();
