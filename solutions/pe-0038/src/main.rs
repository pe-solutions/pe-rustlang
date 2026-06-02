// Pandigital Multiples
// https://projecteuler.net/problem=38

use pe_lib::is_pandigital;

fn solve() -> i32 {
    for index in (1..=9876).rev() {
        let candidate = index * 100_002;
        if is_pandigital(&candidate.to_string()) {
            return candidate;
        }
    }
    panic!("no pandigital found")
}

pe_utils::pe_main!();
