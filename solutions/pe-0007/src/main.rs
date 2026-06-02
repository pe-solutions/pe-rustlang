// 10001st Prime
// https://projecteuler.net/problem=7


fn solve() -> usize {
    const UPPER_LIMIT: usize = 114_319;
    let mut primes = vec![true; UPPER_LIMIT + 1];
    let max_prime = (UPPER_LIMIT as f64).sqrt() as usize;
    sieve_of_eratosthenes(&mut primes, UPPER_LIMIT, max_prime);
    find_nth_prime(&primes, 10001).expect("not enough primes")
}

pe_utils::pe_main!();
fn sieve_of_eratosthenes(primes: &mut Vec<bool>, upper_limit: usize, max_prime: usize) {
    for number in 2..=max_prime {
        if primes[number] {
            for multiple in (number * number..=upper_limit).step_by(number) {
                primes[multiple] = false;
            }
        }
    }
}

fn find_nth_prime(primes: &[bool], n: usize) -> Result<usize, String> {
    let mut count = 0;
    
    for (i, &is_prime) in primes.iter().enumerate() {
        if is_prime {
            count += 1;
            
            if count == n {
                return Ok(i);
            }
        }
    }

    Err("Not enough primes found.".to_string())
}

