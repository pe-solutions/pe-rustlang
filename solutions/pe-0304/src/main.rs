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
