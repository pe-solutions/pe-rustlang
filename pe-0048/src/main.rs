// Self powers
// https://projecteuler.net/problem=48

use num_bigint::BigInt;

fn solve() -> BigInt {
    let sum: BigInt = (1..=1000u32).map(|i| BigInt::from(i).pow(i)).sum();
    let modulus = BigInt::from(10u64).pow(10u32);
    &sum % &modulus
}

fn main() {
    pe_utils::run(48, solve);
}
