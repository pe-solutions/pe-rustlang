// Distinct Primes Factors
// https://projecteuler.net/problem=47

 fn generate_omega_sieve(limit: usize) -> Vec<usize> {
    let mut omega_sieve = vec![0; limit];

    for i in 2..limit {
        if omega_sieve[i] == 0 {
            for j in (i..limit).step_by(i) {
                omega_sieve[j] += 1;
            }
        }
    }

    omega_sieve
}

fn find_first_number() -> usize {
    let limit = 200_000; // Adjust this limit based on your needs
    let required_factors = 4; // Set the required number of distinct prime factors

    let omega_sieve = generate_omega_sieve(limit);

    for i in 1..limit - required_factors {
        let found = (0..required_factors).all(|j| omega_sieve[i + j] == required_factors);

        if found {
            return i;
        }
    }

    0 // If no such number is found
}

fn main() {
    let start = std::time::Instant::now();

    let answer = find_first_number();

    let duration = start.elapsed();

    println!("\nProject Euler #47\nAnswer: {}", answer);
    println!("Elapsed time: {} milliseconds.", duration.as_millis());
}
