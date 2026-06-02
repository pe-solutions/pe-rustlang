// Hyperexponentiation
// https://projecteuler.net/problem=188

use pe_lib::mod_pow;

fn tetra_mod(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut x = base;

    for _ in 2..=exponent {
        x = mod_pow(base, x, modulus);
    }

    x
}

fn solve() -> u64 {
    tetra_mod(1777, 1855, 10u64.pow(8))
}

pe_utils::pe_main!();
