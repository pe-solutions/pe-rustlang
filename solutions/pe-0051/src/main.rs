// Prime Digit Replacements
// https://projecteuler.net/problem=51

use pe_lib::{sieve_primes, is_prime};

fn solve() -> u64 {
    let primes: Vec<usize> = sieve_primes(1_000_000);

    for &p in &primes {
        let s: String = p.to_string();
        let len = s.len();

        // Try each subset of digit positions
        for mask in 1..(1 << len) {
            let mut count = 0;
            let mut first_prime = 0u64;

            for digit in 0..=9 {
                let mut candidate = s.clone();
                let mut valid = true;

                for i in 0..len {
                    if (mask >> i) & 1 == 1 {
                        candidate.replace_range(i..=i, &digit.to_string());
                        if i == 0 && digit == 0 {
                            valid = false;
                            break;
                        }
                    }
                }

                if valid {
                    if let Ok(num) = candidate.parse::<u64>() {
                        if is_prime(num) {
                            if count == 0 {
                                first_prime = num;
                            }
                            count += 1;
                        }
                    }
                }
            }

            if count == 8 {
                return first_prime;
            }
        }
    }

    0
}

pe_utils::pe_main!();
