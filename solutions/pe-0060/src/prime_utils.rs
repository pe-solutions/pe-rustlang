// prime_utils.rs

use pe_lib::{sieve_primes, is_prime};

pub fn find_prime_set() -> Option<Vec<u64>> {
    fn find_concatenated_prime_set(primes: &[u64]) -> Option<Vec<u64>> {
        fn build_prime_chain(chain: Vec<u64>, primes: &[u64]) -> Option<Vec<u64>> {
            if chain.len() == 5 {
                return Some(chain);
            }

            let &last_prime = chain.last().unwrap();
            let next_primes = filter_concatenated_primes(last_prime, primes);

            for &next in &next_primes {
                let mut new_chain = chain.clone();
                new_chain.push(next);

                if let Some(result) = build_prime_chain(new_chain, &next_primes) {
                    return Some(result);
                }
            }

            None
        }

        fn filter_concatenated_primes(x: u64, candidates: &[u64]) -> Vec<u64> {
            fn are_concatenated_primes(x: u64, y: u64) -> bool {
                let concat1: u64 = format!("{}{}", x, y).parse().unwrap();
                let concat2: u64 = format!("{}{}", y, x).parse().unwrap();
                is_prime(concat1) && is_prime(concat2)
            }

            candidates
                .iter()
                .cloned()
                .filter(|&y| y > x && are_concatenated_primes(x, y))
                .collect()
        }

        for &a in primes {
            if let Some(result) = build_prime_chain(vec![a], primes) {
                return Some(result);
            }
        }
        None
    }

    let primes: Vec<u64> = sieve_primes(10000).iter().map(|&p| p as u64).collect();
    find_concatenated_prime_set(&primes)
}
