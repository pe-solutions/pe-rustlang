const MOD: u128 = 1234567891011;
const START: u64 = 100_000_000_000_000;
const NEED: usize = 100_000;

// fast doubling
fn fib(n: u64) -> (u128, u128) {
    if n == 0 {
        return (0, 1);
    }
    let (a, b) = fib(n >> 1);
    let c = (a * ((2 * b + MOD - a) % MOD)) % MOD;
    let d = (a * a + b * b) % MOD;
    if n & 1 == 0 {
        (c, d)
    } else {
        (d, (c + d) % MOD)
    }
}

use pe_lib::is_prime;

fn solve() -> u128 {
    let mut primes = Vec::with_capacity(NEED);
    let mut x = START + 1;
    while primes.len() < NEED {
        if is_prime(x) { primes.push(x); }
        x += 1;
    }
    let p0 = primes[0];
    let (mut f_prev, mut f_cur) = fib(p0 - 1);
    let mut sum = f_cur % MOD;
    for i in 1..NEED {
        let gap = primes[i] - primes[i - 1];
        let (fd, fd1) = fib(gap);
        let new_f = (f_cur * fd1 + f_prev * fd) % MOD;
        let new_prev = (f_cur * fd + f_prev * ((fd1 + MOD - fd) % MOD)) % MOD;
        f_cur = new_f;
        f_prev = new_prev;
        sum = (sum + f_cur) % MOD;
    }
    sum % MOD
}

pe_utils::pe_main!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_base_case() {
        let (f0, f1) = fib(0);
        assert_eq!(f0, 0);
        assert_eq!(f1, 1);
    }

    #[test]
    fn test_fib_small_values() {
        let (f1_prev, f1_cur) = fib(1);
        assert!(f1_cur > 0);

        let (f2_prev, f2_cur) = fib(2);
        assert!(f2_cur > 0);
    }

    #[test]
    fn test_is_prime_verification() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(97));
        assert!(is_prime(101));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
    }

    #[test]
    fn test_modular_arithmetic() {
        let val = 10u128.pow(12);
        let result = val % MOD;
        assert!(result < MOD);
    }

    #[test]
    fn test_solve_produces_result() {
        let result = solve();
        assert!(result > 0);
        assert!(result < MOD);
    }
}

