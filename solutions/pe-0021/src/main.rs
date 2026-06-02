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

pe_utils::pe_main!();
