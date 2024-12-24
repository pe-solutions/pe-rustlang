// Largest Prime Factor
// https://projecteuler.net/problem/3

fn largest_prime_factor(mut n: u64) -> u64 {
    let mut i: u64 = 2;
    while i * i <= n {
        while n % i == 0 {
            n /= i;
        }
        i += 1;
    }
    n
}

fn main() {
    let start = std::time::Instant::now();
    
    let number: u64 = 600_851_475_143;
    let answer = largest_prime_factor(number);

    let duration = start.elapsed();

    println!("\nProject Euler #3\nAnswer: {}", answer);
    println!("Elapsed time: {:?} milliseconds.\n", duration.as_millis());
}
