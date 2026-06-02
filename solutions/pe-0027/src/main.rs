// Quadratic Primes
// https://projecteuler.net/problem=27

use pe_lib::is_prime_trial;

fn is_prime(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    is_prime_trial(n as u64)
}

fn solve() -> i32 {
    let mut max_primes = 0;
    let mut answer = 0;
    for a in -999..=1000 {
        for b in -1000..=1000 {
            let mut n = 0;
            while is_prime(n * n + a * n + b) {
                n += 1;
            }
            if n > max_primes {
                max_primes = n;
                answer = a * b;
            }
        }
    }
    answer
}

pe_utils::pe_main!();
