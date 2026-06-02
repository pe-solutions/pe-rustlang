// Highly Divisible Triangular Number
// https://projecteuler.net/problem=12

use pe_lib::count_divisors;

fn solve() -> u64 {
    let mut result = 0;
    let mut number_to_check = 0;

    while count_divisors(result) <= 500 {
        number_to_check += 1;
        result += number_to_check;
    }

    result
}

pe_utils::pe_main!();
