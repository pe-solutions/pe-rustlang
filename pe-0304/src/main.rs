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

// deterministic Miller-Rabin for u64
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in &[2, 3, 5, 7, 11, 13] {
        if n % p == 0 {
            return n == p;
        }
    }

    let d = (n - 1) >> (n - 1).trailing_zeros();

    fn mod_pow(mut a: u128, mut d: u128, n: u128) -> u128 {
        let mut r = 1;
        while d > 0 {
            if d & 1 == 1 {
                r = r * a % n;
            }
            a = a * a % n;
            d >>= 1;
        }
        r
    }

    for &a in &[2u64, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        if a % n == 0 {
            continue;
        }
        let mut x = mod_pow(a as u128, d as u128, n as u128);
        if x == 1 || x == n as u128 - 1 {
            continue;
        }
        let mut dd = d;
        let mut composite = true;
        while dd != n - 1 {
            x = x * x % n as u128;
            dd <<= 1;
            if x == n as u128 - 1 {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}

fn main() {
    let start = std::time::Instant::now();
    
    //-----------------------------------
    
    // Step 1: primes
    let mut primes = Vec::with_capacity(NEED);
    let mut x = START + 1;

    while primes.len() < NEED {
        if is_prime(x) {
            primes.push(x);
        }
        x += 1;
    }

    // Step 2: initial Fibonacci
    let p0 = primes[0];
    let (mut f_prev, mut f_cur) = fib(p0 - 1); // F(n-1), F(n)

    let mut sum = f_cur % MOD;

    // Step 3: iterate using gaps
    for i in 1..NEED {
        let gap = primes[i] - primes[i - 1];

        let (fd, fd1) = fib(gap); // F(d), F(d+1)

        let new_f = (f_cur * fd1 + f_prev * fd) % MOD;
        let new_prev = (f_cur * fd + f_prev * ((fd1 + MOD - fd) % MOD)) % MOD;

        f_cur = new_f;
        f_prev = new_prev;

        sum = (sum + f_cur) % MOD;
    }
    
    //-----------------------------------
    
    let duration = start.elapsed();

    println!("\nProject Euler #304\nAnswer: {}", sum % MOD);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis()); 
}