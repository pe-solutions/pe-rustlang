// Powers with Trailing Digits
// https://projecteuler.net/problem=455

use pe_lib::mod_pow;

fn f(n: u64, md: u64) -> u64 {
    let mut r1 = n;
    let mut r2 = mod_pow(n, r1, md);

    while r2 != r1 && r2 != 1 {
        r1 = r2;
        r2 = mod_pow(n, r1, md);
    }

    if r2 == 1 { 0 } else { r2 }
}

fn solve() -> u64 {
    let md = 1_000_000_000;
    (2..=1_000_000).map(|n| f(n, md)).sum()
}

pe_utils::pe_main!();
