// Square Root Digital Expansion
// https://projecteuler.net/problem=80

use pe_lib::is_perfect_square;

fn solve() -> u64 {
    let mut sum = 0;
    for n in 1..=100 {
        if !is_perfect_square(n) {
            let mut x = n as u128;
            let mut y = (x + 1) / 2;
            while y < x {
                x = y;
                y = (x + n as u128 / x) / 2;
            }
            let root_str = format!("{:0>100}", x);
            for (i, c) in root_str.chars().take(100).enumerate() {
                if i < 100 {
                    sum += (c as u64) - 48;
                }
            }
        }
    }
    sum
}

pe_utils::pe_main!();
