// Prime permutations
// https://projecteuler.net/problem=49

fn is_prime(n: i32) -> bool {
    if n % 2 == 0 && n != 2 {
        return false;
    }

    for i in 3..=(n as f64).sqrt() as i32 + 1 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn is_permutation(n1: i32, n2: i32) -> bool {
    let mut n1_digits: Vec<char> = n1.to_string().chars().collect();
    let mut n2_digits: Vec<char> = n2.to_string().chars().collect();

    n1_digits.sort();
    n2_digits.sort();

    n1_digits == n2_digits
}

fn prime_list(s: i32, e: i32) -> Vec<i32> {
    (s..=e).filter(|&n| is_prime(n)).collect()
}

fn prime_permutations(primes: &Vec<i32>) -> Vec<i32> {
    for p1 in 0..primes.len() {
        for p2 in (p1 + 1)..primes.len() {
            let d1 = primes[p2] - primes[p1];

            for p3 in (p2 + 1)..primes.len() {
                let d2 = primes[p3] - primes[p2];

                if d1 == d2 && is_permutation(primes[p1], primes[p2]) && is_permutation(primes[p2], primes[p3]) {
                    return vec![primes[p1], primes[p2], primes[p3]];
                }
            }
        }
    }

    vec![]
}

fn main() {
    let start = std::time::Instant::now();

    let primes = prime_list(1_488, 10_000);
    let result = prime_permutations(&primes);

    println!("\nProject Euler #49\nAnswer:{}", result.iter().map(|&n| n.to_string()).collect::<String>());

    let duration = start.elapsed();
    println!("Elapsed time: {} milliseconds.", duration.as_millis());
}
