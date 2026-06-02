// Truncatable Primes
// https://projecteuler.net/problem=37

use pe_lib::is_prime;


fn is_truncatable(t: usize) -> bool {
    let num_str = t.to_string();

    (0..(num_str.len() - 1))
        .all(|i| {
            let left = &num_str[..=i];
            let right = &num_str[i + 1..];
            is_prime(left.parse::<u64>().unwrap()) && is_prime(right.parse::<u64>().unwrap())
        })
}

fn solve() -> usize {
    (10..)
        .filter(|&i| is_prime(i as u64) && is_truncatable(i))
        .take(11)
        .sum::<usize>()
}

pe_utils::pe_main!();
