// Circular Primes
// https://projecteuler.net/problem=35

fn is_prime(n: u32) -> bool {
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    true
}

fn test_rotation_prime(mut n: u32) -> bool {
    let s = n.to_string();
    let mut t = s.clone();

    loop {
        if !is_prime(n) {
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

fn main() {
    pe_utils::run(35, solve);
}
