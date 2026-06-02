pub fn sieve_bools(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit + 1];
    if limit > 0 {
        is_prime[0] = false;
    }
    if limit > 1 {
        is_prime[1] = false;
    }
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
}

pub fn sieve_primes(limit: usize) -> Vec<usize> {
    let is_prime = sieve_bools(limit);
    (2..=limit).filter(|&i| is_prime[i]).collect()
}

pub fn sieve_omega(limit: usize) -> Vec<usize> {
    let mut omega = vec![0; limit];
    for i in 2..limit {
        if omega[i] == 0 {
            for j in (i..limit).step_by(i) {
                omega[j] += 1;
            }
        }
    }
    omega
}
