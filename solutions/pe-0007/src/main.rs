// 10001st Prime
// https://projecteuler.net/problem=7

use pe_lib::sieve_bools;

fn solve() -> usize {
    const UPPER_LIMIT: usize = 114_319;
    let primes = sieve_bools(UPPER_LIMIT);
    find_nth_prime(&primes, 10001).expect("not enough primes")
}

pe_utils::pe_main!();

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

