// Power Digit Sum
// https://projecteuler.net/problem=16

use num_bigint::ToBigUint;

fn solve() -> u32 {
    let base = 2u32.to_biguint().unwrap();
    let power = base.pow(1000);
    let digits_str = power.to_string();

    digits_str.chars().map(|c| c.to_digit(10).unwrap()).sum()
}

pe_utils::pe_main!();
