// prime_utils.rs

pub fn find_prime_set() -> Option<Vec<u64>> {
    fn generate_primes(limit: usize) -> Vec<u64> {
        let mut sieve = vec![true; limit + 1];
        sieve[0] = false;
        sieve[1] = false;

        for i in 2..=limit {
            if sieve[i] {
                let mut multiple = i * i;
                while multiple <= limit {
                    sieve[multiple] = false;
                    multiple += i;
                }
            }
        }

        sieve.into_iter()
            .enumerate()
            .filter_map(|(num, is_prime)| if is_prime { Some(num as u64) } else { None })
            .collect()
    }

    fn is_prime(n: u64) -> bool {
        if n <= 1 {
            return false;
        }
        if n == 2 || n == 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

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
                let concat1 = format!("{}{}", x, y).parse().unwrap();
                let concat2 = format!("{}{}", y, x).parse().unwrap();
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

    let primes = generate_primes(10000);
    find_concatenated_prime_set(&primes)
}
