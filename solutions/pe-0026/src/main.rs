// Reciprocal Cycles
// https://projecteuler.net/problem=26

use std::collections::HashMap;

fn cycle_length(numerator: i32, denominator: i32) -> Result<i32, &'static str> {
    if denominator == 0 {
        return Err("Denominator cannot be zero");
    }

    let mut remainders = HashMap::new();
    let (mut num, mut position) = (numerator, 0);

    while !remainders.contains_key(&num) && num != 0 {
        remainders.insert(num, position);
        num = (num % denominator) * 10;
        position += 1;
    }

    Ok(if num == 0 { 0 } else { position - remainders[&num] })
}

fn solve() -> i32 {
    let mut max_val = 0;
    let mut d_max = 0;
    for i in 2..1_000 {
        if let Ok(len) = cycle_length(1, i) {
            if len > max_val {
                max_val = len;
                d_max = i;
            }
        }
    }
    d_max
}

pe_utils::pe_main!();
