// Even Fibonacci Numbers
// https://projecteuler.net/problem=2

use pe_lib::Fibonacci;

fn sum_of_even_fibo(limit: u64) -> u64 {
    Fibonacci::new()
        .filter(|&x| x % 2 == 0)
        .take_while(|&x| x < limit)
        .sum()
}

fn solve() -> u64 {
    sum_of_even_fibo(4_000_000)
}

pe_utils::pe_main!();
