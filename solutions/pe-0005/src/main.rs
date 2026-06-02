// Smallest Multiple
// https://projecteuler.net/problem/5


fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn solve() -> u64 {
    (1..=20u64).fold(1, |acc, x| lcm(acc, x))
}

pe_utils::pe_main!();
