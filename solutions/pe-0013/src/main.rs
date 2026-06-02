// Large sum
// https://projecteuler.net/problem=13

use num_bigint::BigUint;
use pe_lib::read_lines;

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

pe_utils::pe_main!();
