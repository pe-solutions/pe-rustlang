// Lychrel Numbers
// https://projecteuler.net/problem=55

use pe_lib::{reverse_digits_generic, is_palindrome_generic};

fn is_lychrel(mut number: u128) -> bool {
    for _ in 0..50 {
        number += reverse_digits_generic(number);

        if is_palindrome_generic(number) {
            return false;
        }
    }

    true
}

fn solve() -> u32 {
    let mut lychrel_total = 0;
    for candidate in 1..=10_000 {
        if is_lychrel(candidate as u128) { lychrel_total += 1; }
    }
    lychrel_total
}

pe_utils::pe_main!();
