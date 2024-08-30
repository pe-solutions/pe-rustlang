// Largest Prime
// https://projecteuler.net/problem=659

const MOD: u64 = 1_000_000_000_000_000_000;

fn modmul(a: u64, b: u64, m: u64) -> u64 {
    (((a as u128) * (b as u128)) % (m as u128)) as u64
}

fn modpow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = modmul(result, base, modulus);
        }
        base = modmul(base, base, modulus);
        exp /= 2;
    }
    result
}

fn find_quadratic_nonresidue(p: u64) -> u64 {
    (2..p).find(|&a| modpow(a, p / 2, p) == p - 1).expect("No quadratic non-residue found")
}

fn leastrs(mut t: u64, p: u64) -> u64 {
    (0..p).find(|_| {
        if t == 1 { return true; }
        t = modmul(t, t, p);
        false
    }).expect("No least RS found")
}

fn fp_square_root(n: u64, p: u64) -> Option<u64> {
    if n == 0 { return Some(0); }

    let m = (p - 1).trailing_zeros() as u64;
    let q = (p - 1) >> m;
    let z = find_quadratic_nonresidue(p);

    let mut c = modpow(z, q, p);
    let mut t = modpow(n, q, p);
    let mut r = modpow(n, (q + 1) / 2, p);

    while t != 1 {
        let i = leastrs(t, p);
        if i == m { return None; }

        let b = modpow(c, 1 << (m - i - 1), p);
        r = modmul(r, b, p);
        c = modmul(b, b, p);
        t = modmul(t, c, p);
    }

    Some(r)
}

fn main() {
    let start = std::time::Instant::now();
    
    let n = 10_000_000;
    let mut sieve = (0..n)
        .map(|i| (4 * (i + 1) as u64 * (i + 1) as u64 + 1, 1u64))
        .collect::<Vec<_>>();

    for p in primal::Primes::all().take_while(|&p| p <= 2 * n).filter(|&p| p % 4 == 1) {
        let p = p as u64;
        if let Some(sr) = fp_square_root(p - 1, p) {
            let s = if sr & 1 == 0 { sr / 2 } else { (sr + p) / 2 };
            let offsets = [(s - 1) as usize, (p - s - 1) as usize];

            for i in (0..n).step_by(p as usize) {
                for &offset in &offsets {
                    if i + offset < n {
                        let (mut q, _prime) = sieve[i + offset];
                        while q % p == 0 {
                            q /= p;
                        }
                        sieve[i + offset] = (q, p);
                    }
                }
            }
        }
    }

    let result = sieve.iter()
        .map(|&(a, b)| a.max(b as u64))
        .fold(0u64, |acc, v| (acc + v) % MOD);

    let duration = start.elapsed();
    
    println!("\nProject Euler #659\nAnswer: {}", result);
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis());
}
