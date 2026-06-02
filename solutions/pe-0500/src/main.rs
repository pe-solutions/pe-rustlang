// Problem 500!!!
// https://projecteuler.net/problem=500

use std::collections::BinaryHeap;
use std::cmp::Reverse;
use pe_lib::sieve_primes;

fn solve() -> usize {
    const N: usize = 500500;
    const MOD: usize = 500500507;
    let primes = sieve_primes(7376507);
    let mut pq = BinaryHeap::new();
    for &prime in &primes { pq.push(Reverse(prime)); }
    let mut answer = 1;
    for _ in 0..N {
        let x = pq.pop().unwrap().0;
        answer = (answer * x) % MOD;
        pq.push(Reverse(x * x));
    }
    answer
}

pe_utils::pe_main!();
