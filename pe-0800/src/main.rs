// Hybrid Integers
// https://projecteuler.net/problem=800


fn sieve(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];

    is_prime[2..].iter_mut().for_each(|p| *p = true);

    let sqrt_n = (n as f64).sqrt() as usize + 1;

    for i in 2..sqrt_n {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    let mut primes = Vec::new();

    for i in 2..=n {
        if is_prime[i] {
            primes.push(i);
        }
    }

    primes
}

fn is_valid(p: f64, q: f64, b: f64, e: f64) -> bool {
    p*q.log2()+q*p.log2() <= b*e.log2()
}

fn main() {
    let start = std::time::Instant::now();
    
    const BASE: usize = 800_800;
    const EXP: usize = 800_800;

    let primes = sieve((800_800_f64 * 800_800_f64.log2()) as usize);

    let mut answer = 0;

    for p in 0..primes.len() - 1 {
        for q in p + 1..primes.len() {
            if is_valid(primes[p] as f64, primes[q] as f64, BASE as f64, EXP as f64) {
                answer += 1;
            } else {
                // Inner loop bound
                break;
            }
        }

        if !is_valid(primes[p] as f64, 2.0, BASE as f64, EXP as f64) {
            // Outer loop bound
            break;
        }
    }

    let duration = start.elapsed();

    println!("\nProject Euler #800\nAnswer: {}", answer); // println!("\n{}", total);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis()); 
}
