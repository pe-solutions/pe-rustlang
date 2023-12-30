// Problem 500!!!
// https://projecteuler.net/problem=500

use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn generate_primes(n: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; n + 1];

    for num in 2..=n {
        if is_prime[num] {
            primes.push(num);

            for multiple in (num..=n).step_by(num) {
                is_prime[multiple] = false;
            }
        }
    }

    primes
}

fn main() {
    let start = std::time::Instant::now();
    
    const N: usize = 500500;
    const MOD: usize = 500500507;

    // prime(500500) = 7376507 - Pari/GP's nth prime
    let primes = generate_primes(7376507);

    let mut pq = BinaryHeap::new();
    for &prime in &primes {
        pq.push(Reverse(prime));
    }

    let mut answer = 1;
    for _ in 0..N {
        let x = pq.pop().unwrap().0;

        answer = (answer * x) % MOD;

        pq.push(Reverse(x * x));
    }

    let duration = start.elapsed();

    println!("\nProject Euler #800\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());

}
