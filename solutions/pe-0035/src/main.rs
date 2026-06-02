// Circular Primes
// https://projecteuler.net/problem=35

use pe_lib::is_prime;

fn test_rotation_prime(mut n: u32) -> bool {
    let s = n.to_string();
    let mut t = s.clone();

    loop {
        if !is_prime(n as u64) {
            return false;
        }

        t = t.chars().cycle().skip(1).take(t.len()).collect();

        if let Ok(val) = t.parse::<u32>() {
            n = val;
        } else {
            return false;
        }

        if s == t {
            break;
        }
    }

    true
}

fn solve() -> u32 {
    const UPPERBOUND: u32 = 1_000_000;
    let mut i: u32 = 2;
    let mut answer = 0;
    while i < UPPERBOUND {
        if test_rotation_prime(i) { answer += 1; }
        i += 1;
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_digit_primes() {
        // Single digit circular primes: 2, 3, 5, 7
        assert!(test_rotation_prime(2));
        assert!(test_rotation_prime(3));
        assert!(test_rotation_prime(5));
        assert!(test_rotation_prime(7));
        assert!(!test_rotation_prime(4));
        assert!(!test_rotation_prime(6));
    }

    #[test]
    fn test_known_circular_primes() {
        // 13: rotations are 13 and 31 (both prime)
        assert!(test_rotation_prime(13));
    }

    #[test]
    fn test_non_circular_primes() {
        // 11: rotation gives 11 (same, prime), but let's check
        // 23: rotations are 23 and 32 (32 is not prime)
        assert!(!test_rotation_prime(23));
    }

    #[test]
    fn test_solve_produces_count() {
        let result = solve();
        assert!(result > 0);
        // There are 55 circular primes below 1 million
        assert!(result < 100);
    }
}

pe_utils::pe_main!();
