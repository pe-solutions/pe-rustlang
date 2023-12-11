// Totient Permutation
// https://projecteuler.net/problem=70

pub fn prime_sieve(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];

    is_prime[0] = false;
    is_prime[1] = false;
    
    for i in 2..=(limit as f64).sqrt() as usize {
        if is_prime[i] {
            for j in (i * i..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &is_prime)| if is_prime { Some(i) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prime_sieve() {
        assert_eq!(prime_sieve(10), vec![2, 3, 5, 7]);
        assert_eq!(prime_sieve(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}


fn find_n(lim: usize) -> usize {
    // Generate primes (bounded by sqrt(lim))
    let primes = prime_sieve((2 * (lim as f64).sqrt() as usize) + 1);

    let mut best_ratio = lim as f64;
    let mut best_n = 0;

    
    // Look only into two-distinct-prime composites
    for (i, &p1) in primes.iter().enumerate() {
        for &p2 in &primes[i + 1..] {
            let product = p1 * p2;

            if product > lim {
                break;
            }

            let n = product;
            let phi = (p1 - 1) * (p2 - 1);

            let ratio = n as f64 / phi as f64;

            // Best ration and Permutation
            if ratio < best_ratio && sorted_digits(n) == sorted_digits(phi) {
                best_ratio = ratio;
                best_n = n;
            }
        }
    }

    best_n
}

// Helper function
fn sorted_digits(n: usize) -> Vec<char> {
    let mut digits: Vec<char> = n.to_string().chars().collect();
    digits.sort();

    digits
}

fn main() {
    let start = std::time::Instant::now();
    
    const LIMIT: usize = 10_000_000;
    let answer = find_n(LIMIT);

    let duration = start.elapsed();

    println!("\nProject Euler #70\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
