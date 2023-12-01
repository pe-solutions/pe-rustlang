// Hybrid Integers
// https://projecteuler.net/problem=800

const BASE: usize = 800_800;
const EXP: usize = 800_800;
const LOG_2: f64 = 2.0;

fn sieve(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    let sqrt_n = (n as f64).sqrt() as usize + 1;

    for i in 2..sqrt_n {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    (2..=n).filter(|&i| is_prime[i]).collect()
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

fn main() {
    let start = std::time::Instant::now();

    let primes = sieve((800_800_f64 * 800_800_f64.log2()) as usize);
    let answer = count_valid_combinations(&primes);

    let duration = start.elapsed();

    println!("\nProject Euler #800\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
