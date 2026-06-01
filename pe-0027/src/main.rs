// Quadratic Primes
// https://projecteuler.net/problem=27

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    if n <= 3 {
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

fn solve() -> i32 {
    let mut max_primes = 0;
    let mut answer = 0;
    for a in -999..=1000 {
        for b in -1000..=1000 {
            let mut n = 0;
            while is_prime(n * n + a * n + b) {
                n += 1;
            }
            if n > max_primes {
                max_primes = n;
                answer = a * b;
            }
        }
    }
    answer
}

fn main() {
    pe_utils::run(27, solve);
}
