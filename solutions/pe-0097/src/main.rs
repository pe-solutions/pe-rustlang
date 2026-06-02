// Large Non-Mersenne Prime
// https://projecteuler.net/problem=97

use pe_lib::mod_pow;

fn solve() -> u64 {
    let m = 10_000_000_000u64;
    let mut answer = mod_pow(2, 7_830_457, m);
    answer = (answer * 28_433) % m;
    (answer + 1) % m
}

pe_utils::pe_main!();
