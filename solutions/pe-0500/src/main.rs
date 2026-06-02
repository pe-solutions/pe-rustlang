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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_primes() {
        let primes = sieve_primes(20);
        assert!(primes.len() > 0);
        assert!(primes.contains(&2));
        assert!(primes.contains(&3));
        assert!(primes.contains(&5));
    }

    #[test]
    fn test_priority_queue_order() {
        let mut pq: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
        pq.push(Reverse(5));
        pq.push(Reverse(2));
        pq.push(Reverse(3));
        assert_eq!(pq.pop().unwrap().0, 2);
        assert_eq!(pq.pop().unwrap().0, 3);
        assert_eq!(pq.pop().unwrap().0, 5);
    }

    #[test]
    fn test_modulo_arithmetic() {
        const MOD: usize = 500500507;
        let a = 1234567usize;
        let b = 9876543usize;
        let result = (a * b) % MOD;
        assert!(result < MOD);
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > 0);
        assert!(result < 500500507);
    }
}

pe_utils::pe_main!();
