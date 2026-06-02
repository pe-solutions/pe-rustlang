// Hybrid Integers
// https://projecteuler.net/problem=800

use pe_lib::sieve_primes;

const BASE: usize = 800_800;
const EXP: usize = 800_800;
const LOG_2: f64 = 2.0;

fn sieve(n: usize) -> Vec<usize> {
    sieve_primes(n)
}

struct Parameters {
    p: f64,
    q: f64,
    b: f64,
    e: f64,
}

fn is_valid(params: &Parameters) -> bool {
    params.p * params.q.log2() + params.q * params.p.log2() <= params.b * params.e.log2()
}

fn count_valid_combinations(primes: &Vec<usize>) -> usize {
    let mut valid_combinations = 0;

    for (p, &prime_p) in primes.iter().enumerate().take(primes.len() - 1) {
        for &prime_q in primes.iter().skip(p + 1) {
            if is_valid(&Parameters { p: prime_p as f64, q: prime_q as f64, b: BASE as f64, e: EXP as f64 }) {
                valid_combinations += 1;
            } else {
                // Inner loop bound
                break;
            }
        }

        if !is_valid(&Parameters { p: prime_p as f64, q: LOG_2, b: BASE as f64, e: EXP as f64 }) {
            // Outer loop bound
            break;
        }
    }

    valid_combinations
}

fn solve() -> usize {
    let primes = sieve((800_800_f64 * 800_800_f64.log2()) as usize);
    count_valid_combinations(&primes)
}

pe_utils::pe_main!();
