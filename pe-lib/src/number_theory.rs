use crate::divisors::prime_factors;
use crate::sieve::sieve_bools;

pub fn totient(n: u64) -> u64 {
    if n == 1 {
        return 1;
    }
    let factors = prime_factors(n);
    let mut result = n;
    for (p, _) in factors {
        result = result / p * (p - 1);
    }
    result
}

pub fn totient_sieve(limit: usize) -> Vec<usize> {
    let mut phi = (0..=limit).collect::<Vec<_>>();
    let is_prime = sieve_bools(limit);

    for i in 2..=limit {
        if is_prime[i] {
            for j in (i..=limit).step_by(i) {
                phi[j] -= phi[j] / (i as usize);
            }
        }
    }
    phi
}
